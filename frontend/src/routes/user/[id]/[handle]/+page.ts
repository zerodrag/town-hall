import { error, redirect } from '@sveltejs/kit';
import type { User } from '$lib/backend/generated-types.js';
import { getUser } from '$lib/backend/user.js';
import { resolve } from '$app/paths';

export const load = async ({ params, fetch, route }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, await result.text());
  const targetUser: User = await result.json();
  if (targetUser.handle !== params.handle) {
    redirect(308, resolve(route.id, {
      ...params,
      handle: targetUser.handle
    }));
  }
  return { targetUser };
};
