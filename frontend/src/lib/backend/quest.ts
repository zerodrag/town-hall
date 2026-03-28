import { BACKEND_URL, handleResponse, type BackendResult } from './common';
import type { Quest } from './generated-types';

export async function createQuest(
	customFetch: typeof fetch,
	title: string
): Promise<BackendResult<number>> {
	const resp = await customFetch(`${BACKEND_URL}/quests`, {
		method: `POST`,
		headers: { 'Content-Type': 'application/json' },
		body: JSON.stringify(title)
	});
	return await handleResponse(resp);
}

export async function getQuest(
	customFetch: typeof fetch,
	id: string
): Promise<BackendResult<Quest>> {
	const resp = await customFetch(`${BACKEND_URL}/quests/${id}`);
	return await handleResponse(resp);
}
