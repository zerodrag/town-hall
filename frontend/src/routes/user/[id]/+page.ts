import { error, redirect } from '@sveltejs/kit';
import type { User } from '$lib/backend/generated-types.js';
import { getUser } from '$lib/backend/user';

export const load = async ({ fetch, params }) => {
  const resp = await getUser(fetch, params.id);
  if (!resp.ok) error(resp.status, await resp.text());
  const targetUser: User = await resp.json();
  redirect(308, `/user/${targetUser.userId}/${targetUser.handle}`);
};
