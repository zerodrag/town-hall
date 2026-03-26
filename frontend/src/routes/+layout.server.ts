import { getUserMe } from '$lib/backend.js';

export const load = async ({ fetch }) => {
	const user = await getUserMe(fetch);
	if (!user.ok) {
		return { user: null };
	}
	return { user: user.data };
};
