import { fetchBackend } from './common';
import type { CreateQuestRequest, UpdateQuestRequest } from './generated-types';

export async function getQuest(customFetch: typeof fetch, id: string): Promise<Response> {
  return await fetchBackend(customFetch, `/quests/${id}`);
}

/// Returns quest id
export async function createQuest(params: CreateQuestRequest): Promise<Response> {
  return await fetchBackend(fetch, '/quests', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}

export async function updateQuest(id: string, params: UpdateQuestRequest) {
  return await fetchBackend(fetch, `/quests/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}
