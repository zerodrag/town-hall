import type { User } from '$lib/backend/generated-types.js';
import { getUserMe } from '$lib/backend/user.js';

export const load = async ({ fetch }) => {
  const resp = await getUserMe(fetch);
  if (!resp.ok) {
    return { user: null };
  }
  const me: User = await resp.json();
  return { me };
};
