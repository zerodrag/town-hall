import type { SearchQuestResult } from '$lib/backend/generated-types';
import { discoverQuests } from '$lib/backend/quest';

export async function load({ fetch, url }) {
  const query = url.searchParams.get('query');
  const page = url.searchParams.get('page');
  const limit = url.searchParams.get('limit');
  const techs = url.searchParams.getAll('techs');

  const params = {
    query: query || null,
    page: page ? parseInt(page, 10) : null,
    limit: limit ? parseInt(limit, 10) : null,
    techs: techs.length > 0 ? techs : null
  };

  const resp = await discoverQuests(fetch, params);
  if (!resp.ok) {
    return { total: 0, quests: [] };
  }
  const result: SearchQuestResult = await resp.json();
  return result;
}
