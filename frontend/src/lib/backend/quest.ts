import { fetchBackend } from './common';
import type {
  CreateQuestRequest,
  DiscoverQuestParams,
  GetUserQuestParams,
  UpdateQuestRequest
} from './generated-types';

export async function getQuest(customFetch: typeof fetch, id: string): Promise<Response> {
  return await fetchBackend(customFetch, `/quests/${id}`);
}

export async function getQuestFromUser(
  customFetch: typeof fetch,
  userId: string,
  params: GetUserQuestParams
): Promise<Response> {
  return await fetchBackend(customFetch, `/quests/user/${userId}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}

export async function createQuest(customFetch: typeof fetch, params: CreateQuestRequest): Promise<Response> {
  return await fetchBackend(customFetch, '/quests', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}

export async function updateQuest(customFetch: typeof fetch, id: string, params: UpdateQuestRequest) {
  return await fetchBackend(customFetch, `/quests/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}

export async function discoverQuests(customFetch: typeof fetch, params: DiscoverQuestParams): Promise<Response> {
  return await fetchBackend(customFetch, `/discover/quests`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params)
  });
}
