import { BACKEND_URL, handleResponse, type BackendResult } from './common';
import type { CreateQuestRequest, Quest } from './generated-types';

export async function postQuest(
	customFetch: typeof fetch,
	title: string,
	description: string
): Promise<BackendResult<number>> {
	const request: CreateQuestRequest = { title, description };
	const resp = await customFetch(`${BACKEND_URL}/quests`, {
		method: `POST`,
		headers: { 'Content-Type': 'application-json' },
		body: JSON.stringify(request)
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
