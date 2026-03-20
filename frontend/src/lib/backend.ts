import type { User } from './backend-types';

export const BACKEND_URL = 'http://localhost:3000';

export async function getUserMe(customFetch: typeof fetch): Promise<User | null> {
	const response = await customFetch(new URL('/users/me', BACKEND_URL));
	if (!response.ok) return null;
	const user: User = await response.json();
	return user;
}
