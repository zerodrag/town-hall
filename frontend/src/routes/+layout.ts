import type { User } from '$lib/backend/generated-types.js';
import { getUserMe } from '$lib/backend/user.js';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch }) => {
  const resp = await getUserMe(fetch);
  if (!resp.ok) {
    return { me: null };
  }
  const me: User = await resp.json();
  return { me };
};
