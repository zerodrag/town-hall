import { error, redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { User } from '$lib/backend/generated-types.js';
import { getUser } from '$lib/backend/user.js';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch, route }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, await result.text());
  const targetUser: User = await result.json();
  if (targetUser.handle !== params.handle) {
    redirect(
      308,
      resolve(route.id, {
        ...params,
        handle: targetUser.handle
      })
    );
  }
  return { targetUser };
};
