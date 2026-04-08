import { error, redirect } from '@sveltejs/kit';
import { getUser } from '$lib/backend/user';

export const load = async ({ fetch, params }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, result.body);
  const targetUser = result.data;
  redirect(308, `/user/${targetUser.userId}/${targetUser.handle}`);
};
