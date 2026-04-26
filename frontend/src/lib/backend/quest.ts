import { fetchBackend } from './common';
import type { CreateQuestRequest, SearchQuestParams, UpdateQuestRequest } from './generated-types';

export async function getQuest(customFetch: typeof fetch, id: string): Promise<Response> {
  return await fetchBackend(customFetch, `/quests/${id}`);
}

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

export async function discoverQuests(customFetch: typeof fetch, params: SearchQuestParams): Promise<Response> {
  const searchParams = new URLSearchParams();
  if (params.query) searchParams.set('query', params.query);
  if (params.page) searchParams.set('page', String(params.page));
  if (params.limit) searchParams.set('limit', String(params.limit));
  params.techs?.forEach((tech) => searchParams.append('techs', tech));
  const queryString = searchParams.toString();
  return await fetchBackend(customFetch, `/discover/quests${queryString ? `?${queryString}` : ''}`);
}
