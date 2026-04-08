import { error, redirect } from '@sveltejs/kit';
import type { User } from '$lib/backend/generated-types.js';
import { getUser } from '$lib/backend/user.js';

export const load = async ({ params, fetch }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, await result.text());
  const targetUser: User = await result.json();
  if (targetUser.handle !== params.handle) {
    redirect(308, `/user/${targetUser.userId}/${targetUser.handle}`);
  }
  return { targetUser };
};
