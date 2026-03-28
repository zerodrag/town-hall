import { error, redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {
	const { user } = await parent();
	if (!user) {
		error(401, 'Not logged in');
	}
	redirect(302, `/user/${user.user_id}/${user.handle}`);
};
