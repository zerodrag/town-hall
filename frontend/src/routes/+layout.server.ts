import { getUserMe } from '$lib/backend.js';

export const load = async ({ fetch }) => {
	const user = await getUserMe(fetch);
	return { user };
};
