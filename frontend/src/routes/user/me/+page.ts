import { error, redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent }) => {
  const { me } = await parent();
  if (!me) {
    error(401, 'Not logged in');
  }
  redirect(307, `/user/${me.userId}/${me.handle}`);
};
