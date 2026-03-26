import { getUserFromHandle } from '$lib/backend.js';
import { error } from '@sveltejs/kit';

export const load = async ({ params, fetch }) => {
	const targetUserResult = await getUserFromHandle(fetch, params.handle);
	if (!targetUserResult.ok) error(targetUserResult.status, targetUserResult.body);
	const targetUser = targetUserResult.data;
	return { targetUser };
};
