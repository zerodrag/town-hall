import { betterAuth } from 'better-auth';
import { Pool } from 'pg';
import { DATABASE_URL, GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET } from '$env/static/private';

export const auth = betterAuth({
	database: new Pool({
		connectionString: DATABASE_URL
	}),
	emailAndPassword: {
		enabled: false
	},
	socialProviders: {
		github: {
			clientId: GITHUB_CLIENT_ID,
			clientSecret: GITHUB_CLIENT_SECRET
		}
	},
	user: {
		fields: {
			emailVerified: 'email_verified',
			createdAt: 'created_at',
			updatedAt: 'updated_at'
		}
	},
	session: {
		fields: {
			userId: 'user_id',
			expiresAt: 'expires_at',
			ipAddress: 'ip_address',
			userAgent: 'user_agent',
			createdAt: 'created_at',
			updatedAt: 'updated_at'
		}
	},
	account: {
		fields: {
			userId: 'user_id',
			accountId: 'account_id',
			providerId: 'provider_id',
			accessToken: 'access_token',
			refreshToken: 'refresh_token',
			accessTokenExpiresAt: 'access_token_expires_at',
			refreshTokenExpiresAt: 'refresh_token_expires_at',
			idToken: 'id_token',
			createdAt: 'created_at',
			updatedAt: 'updated_at'
		}
	},
	verification: {
		fields: {
			expiresAt: 'expires_at',
			createdAt: 'created_at',
			updatedAt: 'updated_at'
		}
	}
});
