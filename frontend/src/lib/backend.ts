import type { User } from './backend-types';

export const BACKEND_URL = 'http://localhost:3000';

type BackendResult<T> = { ok: true; data: T } | { ok: false; status: number; body: string };

export async function getUserMe(customFetch: typeof fetch): Promise<BackendResult<User>> {
	return await fetchApi<User>(customFetch, `${BACKEND_URL}/users/me`);
}

export async function getUser(customFetch: typeof fetch, id: string): Promise<BackendResult<User>> {
	return await fetchApi<User>(customFetch, `${BACKEND_URL}/users/${id}`);
}

export async function getUserFromHandle(
	customFetch: typeof fetch,
	handle: string
): Promise<BackendResult<User>> {
	const idResult = await resolveHandle(customFetch, handle);
	if (!idResult.ok) {
		return idResult; // Return the error if id fetch failed
	}
	return await getUser(customFetch, idResult.data);
}

// Helper to simplify URL -> Result
async function fetchApi<T>(customFetch: typeof fetch, url: string): Promise<BackendResult<T>> {
	const response = await customFetch(url);
	if (!response.ok) {
		const body = await response.text();
		return { ok: false, status: response.status, body };
	} else {
		const data: T = await response.json();
		return { ok: true, data };
	}
}

// Helper to fetch ID from handle
async function resolveHandle(
	customFetch: typeof fetch,
	handle: string
): Promise<BackendResult<string>> {
	return await fetchApi<string>(customFetch, `${BACKEND_URL}/users/resolve/${handle}`);
}
