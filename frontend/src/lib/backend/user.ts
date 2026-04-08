import { fetchBackend } from './common';

export async function getUserMe(customFetch: typeof fetch): Promise<Response> {
  return await fetchBackend(customFetch, '/users/me');
}

export async function getUser(customFetch: typeof fetch, id: string): Promise<Response> {
  return await fetchBackend(customFetch, `/users/${id}`);
}

export async function getUserFromHandle(customFetch: typeof fetch, handle: string): Promise<Response> {
  const idResult = await resolveHandle(customFetch, handle);
  if (!idResult.ok) {
    return idResult; // Return the error if id fetch failed
  }
  const id: string = await idResult.json();
  return await getUser(customFetch, id);
}

// Helper to fetch ID from handle
async function resolveHandle(customFetch: typeof fetch, handle: string): Promise<Response> {
  return await fetchBackend(customFetch, `/users/resolve/${handle}`);
}
