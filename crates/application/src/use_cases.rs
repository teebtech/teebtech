//! # Application Use Cases
//!
//! Direct, non-CQRS business workflows orchestrating domain models and repository ports.

use crate::dtos::{
    AuthenticatedUserDto, ClientSessionDto, CreateTenantDto, CurrentUserDto, InviteMemberDto,
    LoginDto, MemberDto, RegisterUserDto, TenantDto, UserTenantMembershipDto,
};
use auth::{PasswordService, TokenService};
use domain::{
    DisplayName, DomainError, Member, MemberEmail, MemberId, MemberName, MemberRepository,
    MemberRole, PasswordHash, PlainPassword, Tenant, TenantId, TenantName, TenantRepository,
    TenantSlug, User, UserEmail, UserId, UserRepository,
};
use shared::AppResult;

/// Use case for registering a new tenant organization.
pub struct RegisterTenantUseCase<'a, R: TenantRepository> {
    tenant_repo: &'a R,
}

impl<'a, R: TenantRepository> RegisterTenantUseCase<'a, R> {
    pub fn new(tenant_repo: &'a R) -> Self {
        Self { tenant_repo }
    }

    pub async fn execute(&self, dto: CreateTenantDto) -> AppResult<TenantDto> {
        let name = TenantName::new(&dto.name)?;
        let slug = TenantSlug::new(&dto.slug)?;

        if let Some(_) = self.tenant_repo.find_by_slug(&slug).await? {
            return Err(DomainError::TenantSlugAlreadyTaken(slug.to_string()).into());
        }

        let id = TenantId::generate();
        let tenant = Tenant::new(id, name, slug);
        self.tenant_repo.save(&tenant).await?;

        Ok(TenantDto::from(tenant))
    }
}

/// Use case for retrieving all registered tenant organizations.
pub struct ListTenantsUseCase<'a, R: TenantRepository> {
    tenant_repo: &'a R,
}

impl<'a, R: TenantRepository> ListTenantsUseCase<'a, R> {
    pub fn new(tenant_repo: &'a R) -> Self {
        Self { tenant_repo }
    }

    pub async fn execute(&self) -> AppResult<Vec<TenantDto>> {
        let tenants = self.tenant_repo.list_all().await?;
        Ok(tenants.into_iter().map(TenantDto::from).collect())
    }
}

/// Use case for inviting a new team member to a tenant organization.
pub struct InviteMemberUseCase<'a, TR: TenantRepository, MR: MemberRepository> {
    tenant_repo: &'a TR,
    member_repo: &'a MR,
}

impl<'a, TR: TenantRepository, MR: MemberRepository> InviteMemberUseCase<'a, TR, MR> {
    pub fn new(tenant_repo: &'a TR, member_repo: &'a MR) -> Self {
        Self {
            tenant_repo,
            member_repo,
        }
    }

    pub async fn execute(&self, dto: InviteMemberDto) -> AppResult<MemberDto> {
        let tenant_id = TenantId::new(&dto.tenant_id);

        if self.tenant_repo.find_by_id(&tenant_id).await?.is_none() {
            return Err(DomainError::TenantNotFound(dto.tenant_id).into());
        }

        let email = MemberEmail::new(&dto.email)?;
        let name = MemberName::new(&dto.name)?;
        let role = MemberRole::from_str_loose(&dto.role)?;

        if self
            .member_repo
            .find_by_tenant_and_email(&tenant_id, &email)
            .await?
            .is_some()
        {
            return Err(DomainError::MemberAlreadyExistsInTenant(dto.email).into());
        }

        let id = MemberId::generate();
        let member = Member::new(id, tenant_id, email, name, role);
        self.member_repo.save(&member).await?;

        Ok(MemberDto::from(member))
    }
}

/// Use case for listing all team members belonging to a tenant organization.
pub struct ListTenantMembersUseCase<'a, MR: MemberRepository> {
    member_repo: &'a MR,
}

impl<'a, MR: MemberRepository> ListTenantMembersUseCase<'a, MR> {
    pub fn new(member_repo: &'a MR) -> Self {
        Self { member_repo }
    }

    pub async fn execute(&self, tenant_id: &str) -> AppResult<Vec<MemberDto>> {
        let tid = TenantId::new(tenant_id);
        let members = self.member_repo.list_by_tenant(&tid).await?;
        Ok(members.into_iter().map(MemberDto::from).collect())
    }
}

/// Use case for registering a new user identity with secure password hashing.
pub struct RegisterUserUseCase<'a, UR: UserRepository> {
    user_repo: &'a UR,
    jwt_secret: &'a str,
}

impl<'a, UR: UserRepository> RegisterUserUseCase<'a, UR> {
    pub fn new(user_repo: &'a UR, jwt_secret: &'a str) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    pub async fn execute(&self, dto: RegisterUserDto) -> AppResult<AuthenticatedUserDto> {
        let email = UserEmail::new(&dto.email)?;
        let plain_password = PlainPassword::new(&dto.password)?;
        let display_name = DisplayName::new(&dto.display_name)?;

        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(DomainError::UserEmailAlreadyRegistered(dto.email).into());
        }

        let hashed = PasswordService::hash_password(plain_password.as_str())?;
        let password_hash = PasswordHash::new(hashed)?;

        let id = UserId::generate();
        let user = User::new(
            id.clone(),
            email.clone(),
            password_hash,
            display_name.clone(),
        );
        self.user_repo.save(&user).await?;

        let token = TokenService::create_session_token(
            id.as_str(),
            email.as_str(),
            self.jwt_secret,
            86400, // 24 hours
        )?;

        Ok(AuthenticatedUserDto {
            id: id.to_string(),
            email: email.to_string(),
            display_name: display_name.to_string(),
            token: token.access_token,
            expires_in: token.expires_in,
        })
    }
}

/// Use case for authenticating user credentials.
pub struct LoginUseCase<'a, UR: UserRepository> {
    user_repo: &'a UR,
    jwt_secret: &'a str,
}

impl<'a, UR: UserRepository> LoginUseCase<'a, UR> {
    pub fn new(user_repo: &'a UR, jwt_secret: &'a str) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    pub async fn execute(&self, dto: LoginDto) -> AppResult<AuthenticatedUserDto> {
        let email = UserEmail::new(&dto.email)?;

        let user = self
            .user_repo
            .find_by_email(&email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        let is_valid =
            PasswordService::verify_password(&dto.password, user.password_hash.as_str())?;
        if !is_valid {
            return Err(DomainError::InvalidCredentials.into());
        }

        let token = TokenService::create_session_token(
            user.id.as_str(),
            user.email.as_str(),
            self.jwt_secret,
            86400,
        )?;

        Ok(AuthenticatedUserDto {
            id: user.id.to_string(),
            email: user.email.to_string(),
            display_name: user.display_name.to_string(),
            token: token.access_token,
            expires_in: token.expires_in,
        })
    }
}

/// Use case for resolving the current authenticated user identity from a session token.
pub struct GetCurrentUserUseCase<'a, UR: UserRepository> {
    user_repo: &'a UR,
    jwt_secret: &'a str,
}

impl<'a, UR: UserRepository> GetCurrentUserUseCase<'a, UR> {
    pub fn new(user_repo: &'a UR, jwt_secret: &'a str) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    pub async fn execute(&self, token: &str) -> AppResult<Option<CurrentUserDto>> {
        let claims = match TokenService::verify_session_token(token, self.jwt_secret) {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };

        let user_id = UserId::new(claims.sub);
        let user_opt = self.user_repo.find_by_id(&user_id).await?;

        Ok(user_opt.map(CurrentUserDto::from))
    }
}

/// Use case for resolving the client portal session context (authenticated user + authorized tenant memberships).
pub struct GetClientSessionUseCase<
    'a,
    UR: UserRepository,
    MR: MemberRepository,
    TR: TenantRepository,
> {
    user_repo: &'a UR,
    member_repo: &'a MR,
    tenant_repo: &'a TR,
    jwt_secret: &'a str,
}

impl<'a, UR: UserRepository, MR: MemberRepository, TR: TenantRepository>
    GetClientSessionUseCase<'a, UR, MR, TR>
{
    pub fn new(
        user_repo: &'a UR,
        member_repo: &'a MR,
        tenant_repo: &'a TR,
        jwt_secret: &'a str,
    ) -> Self {
        Self {
            user_repo,
            member_repo,
            tenant_repo,
            jwt_secret,
        }
    }

    pub async fn execute(&self, token: &str) -> AppResult<Option<ClientSessionDto>> {
        let claims = match TokenService::verify_session_token(token, self.jwt_secret) {
            Ok(c) => c,
            Err(_) => return Ok(None),
        };

        let user_id = UserId::new(claims.sub);
        let user = match self.user_repo.find_by_id(&user_id).await? {
            Some(u) => u,
            None => return Ok(None),
        };

        let member_email = MemberEmail::new(user.email.as_str())?;
        let memberships = self.member_repo.list_by_email(&member_email).await?;

        let mut user_memberships = Vec::new();
        for m in memberships {
            if let Some(tenant) = self.tenant_repo.find_by_id(&m.tenant_id).await? {
                user_memberships.push(UserTenantMembershipDto {
                    tenant_id: tenant.id.to_string(),
                    tenant_name: tenant.name.to_string(),
                    tenant_slug: tenant.slug.to_string(),
                    role: m.role.to_string(),
                });
            }
        }

        Ok(Some(ClientSessionDto {
            user: CurrentUserDto::from(user),
            memberships: user_memberships,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use shared::AppError;
    use std::sync::Mutex;

    struct MockTenantRepo {
        tenants: Mutex<Vec<Tenant>>,
    }

    impl MockTenantRepo {
        fn new() -> Self {
            Self {
                tenants: Mutex::new(Vec::new()),
            }
        }
    }

    impl TenantRepository for MockTenantRepo {
        fn save<'a>(
            &'a self,
            tenant: &'a Tenant,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>
        {
            Box::pin(async move {
                let mut lock = self.tenants.lock().unwrap();
                lock.push(tenant.clone());
                Ok(())
            })
        }

        fn find_by_id<'a>(
            &'a self,
            id: &'a TenantId,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>,
        > {
            Box::pin(async move {
                let lock = self.tenants.lock().unwrap();
                Ok(lock.iter().find(|t| &t.id == id).cloned())
            })
        }

        fn find_by_slug<'a>(
            &'a self,
            slug: &'a TenantSlug,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = AppResult<Option<Tenant>>> + Send + 'a>,
        > {
            Box::pin(async move {
                let lock = self.tenants.lock().unwrap();
                Ok(lock.iter().find(|t| &t.slug == slug).cloned())
            })
        }

        fn list_all<'a>(
            &'a self,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Tenant>>> + Send + 'a>>
        {
            Box::pin(async move {
                let lock = self.tenants.lock().unwrap();
                Ok(lock.clone())
            })
        }
    }

    struct MockMemberRepo {
        members: Mutex<Vec<Member>>,
    }

    impl MockMemberRepo {
        fn new() -> Self {
            Self {
                members: Mutex::new(Vec::new()),
            }
        }
    }

    impl MemberRepository for MockMemberRepo {
        fn save<'a>(
            &'a self,
            member: &'a Member,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>
        {
            Box::pin(async move {
                let mut lock = self.members.lock().unwrap();
                lock.push(member.clone());
                Ok(())
            })
        }

        fn find_by_id<'a>(
            &'a self,
            id: &'a MemberId,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>,
        > {
            Box::pin(async move {
                let lock = self.members.lock().unwrap();
                Ok(lock.iter().find(|m| &m.id == id).cloned())
            })
        }

        fn find_by_tenant_and_email<'a>(
            &'a self,
            tenant_id: &'a TenantId,
            email: &'a MemberEmail,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = AppResult<Option<Member>>> + Send + 'a>,
        > {
            Box::pin(async move {
                let lock = self.members.lock().unwrap();
                Ok(lock
                    .iter()
                    .find(|m| &m.tenant_id == tenant_id && &m.email == email)
                    .cloned())
            })
        }

        fn list_by_tenant<'a>(
            &'a self,
            tenant_id: &'a TenantId,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>
        {
            Box::pin(async move {
                let lock = self.members.lock().unwrap();
                Ok(lock
                    .iter()
                    .filter(|m| &m.tenant_id == tenant_id)
                    .cloned()
                    .collect())
            })
        }

        fn list_by_email<'a>(
            &'a self,
            email: &'a MemberEmail,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Vec<Member>>> + Send + 'a>>
        {
            Box::pin(async move {
                let lock = self.members.lock().unwrap();
                Ok(lock.iter().filter(|m| &m.email == email).cloned().collect())
            })
        }

        fn delete<'a>(
            &'a self,
            id: &'a MemberId,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>
        {
            Box::pin(async move {
                let mut lock = self.members.lock().unwrap();
                lock.retain(|m| &m.id != id);
                Ok(())
            })
        }
    }

    struct MockUserRepo {
        users: Mutex<Vec<User>>,
    }

    impl MockUserRepo {
        fn new() -> Self {
            Self {
                users: Mutex::new(Vec::new()),
            }
        }
    }

    impl UserRepository for MockUserRepo {
        fn save<'a>(
            &'a self,
            user: &'a User,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<()>> + Send + 'a>>
        {
            Box::pin(async move {
                let mut lock = self.users.lock().unwrap();
                if let Some(pos) = lock.iter().position(|u| u.id == user.id) {
                    lock[pos] = user.clone();
                } else {
                    lock.push(user.clone());
                }
                Ok(())
            })
        }

        fn find_by_id<'a>(
            &'a self,
            id: &'a UserId,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<User>>> + Send + 'a>>
        {
            Box::pin(async move {
                let lock = self.users.lock().unwrap();
                Ok(lock.iter().find(|u| &u.id == id).cloned())
            })
        }

        fn find_by_email<'a>(
            &'a self,
            email: &'a UserEmail,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = AppResult<Option<User>>> + Send + 'a>>
        {
            Box::pin(async move {
                let lock = self.users.lock().unwrap();
                Ok(lock.iter().find(|u| &u.email == email).cloned())
            })
        }
    }

    #[tokio::test]
    async fn test_auth_workflow_register_login_current_user() {
        let repo = MockUserRepo::new();
        let jwt_secret = "dev_secret_key_minimum_32_bytes_long";

        let register_case = RegisterUserUseCase::new(&repo, jwt_secret);
        let auth_res = register_case
            .execute(RegisterUserDto {
                email: "alice@teebtech.com".to_string(),
                password: "SecurePassword123!".to_string(),
                display_name: "Alice Engineer".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(auth_res.email, "alice@teebtech.com");
        assert_eq!(auth_res.display_name, "Alice Engineer");
        assert!(!auth_res.token.is_empty());

        // Login with valid credentials
        let login_case = LoginUseCase::new(&repo, jwt_secret);
        let login_res = login_case
            .execute(LoginDto {
                email: "alice@teebtech.com".to_string(),
                password: "SecurePassword123!".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(login_res.id, auth_res.id);

        // Login with invalid credentials fails
        let login_err = login_case
            .execute(LoginDto {
                email: "alice@teebtech.com".to_string(),
                password: "WrongPassword!".to_string(),
            })
            .await
            .unwrap_err();
        assert!(matches!(login_err, AppError::Unauthorized(_)));

        // Get current user via token
        let current_case = GetCurrentUserUseCase::new(&repo, jwt_secret);
        let current_user = current_case.execute(&login_res.token).await.unwrap();
        assert!(current_user.is_some());
        assert_eq!(current_user.unwrap().display_name, "Alice Engineer");
    }

    #[tokio::test]
    async fn test_get_client_session_with_authorized_memberships() {
        let user_repo = MockUserRepo::new();
        let member_repo = MockMemberRepo::new();
        let tenant_repo = MockTenantRepo::new();
        let jwt_secret = "dev_secret_key_minimum_32_bytes_long";

        // 1. Register user
        let register_case = RegisterUserUseCase::new(&user_repo, jwt_secret);
        let auth_res = register_case
            .execute(RegisterUserDto {
                email: "bob@acme.com".to_string(),
                password: "SecurePassword123!".to_string(),
                display_name: "Bob Builder".to_string(),
            })
            .await
            .unwrap();

        // 2. Create tenant
        let tid = TenantId::generate();
        tenant_repo
            .save(&Tenant::new(
                tid.clone(),
                TenantName::new("Acme Corp").unwrap(),
                TenantSlug::new("acme-corp").unwrap(),
            ))
            .await
            .unwrap();

        // 3. Add Bob to Acme Corp
        member_repo
            .save(&Member::new(
                MemberId::generate(),
                tid.clone(),
                MemberEmail::new("bob@acme.com").unwrap(),
                MemberName::new("Bob Builder").unwrap(),
                MemberRole::Admin,
            ))
            .await
            .unwrap();

        // 4. Query client session
        let session_case =
            GetClientSessionUseCase::new(&user_repo, &member_repo, &tenant_repo, jwt_secret);
        let session = session_case
            .execute(&auth_res.token)
            .await
            .unwrap()
            .expect("Session should exist");

        assert_eq!(session.user.email, "bob@acme.com");
        assert_eq!(session.memberships.len(), 1);
        assert_eq!(session.memberships[0].tenant_name, "Acme Corp");
        assert_eq!(session.memberships[0].role, "Admin");
    }

    #[tokio::test]
    async fn test_register_tenant_use_case_success() {
        let repo = MockTenantRepo::new();
        let use_case = RegisterTenantUseCase::new(&repo);

        let res = use_case
            .execute(CreateTenantDto {
                name: "Acme Corp".to_string(),
                slug: "acme-corp".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(res.name, "Acme Corp");
        assert_eq!(res.slug, "acme-corp");
    }

    #[tokio::test]
    async fn test_invite_member_use_case_success() {
        let tenant_repo = MockTenantRepo::new();
        let member_repo = MockMemberRepo::new();

        let tid = TenantId::generate();
        tenant_repo
            .save(&Tenant::new(
                tid.clone(),
                TenantName::new("Acme Corp").unwrap(),
                TenantSlug::new("acme-corp").unwrap(),
            ))
            .await
            .unwrap();

        let use_case = InviteMemberUseCase::new(&tenant_repo, &member_repo);
        let res = use_case
            .execute(InviteMemberDto {
                tenant_id: tid.to_string(),
                email: "bob@acme.com".to_string(),
                name: "Bob Builder".to_string(),
                role: "Admin".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(res.email, "bob@acme.com");
        assert_eq!(res.role, "Admin");
    }
}
