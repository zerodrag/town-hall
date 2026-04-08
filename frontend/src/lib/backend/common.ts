export const BACKEND_URL = 'http://localhost:3000';

export async function fetchBackend(customFetch: typeof fetch, path: string, init?: RequestInit) {
  return await customFetch(`${BACKEND_URL}${path}`, {
    ...init,
    credentials: 'include'
  });
}
