import type { SearchQuestParams, SearchQuestResult } from '$lib/backend/generated-types.js';
import { discoverQuests } from '$lib/backend/quest';

export async function load({ fetch, url }) {
  const query = url.searchParams.get('query');
  const page = url.searchParams.get('page');
  const limit = url.searchParams.get('limit');
  const techs = url.searchParams.getAll('techs');

  const params: SearchQuestParams = {
    query: query || null,
    page: page ? parseInt(page, 10) : null,
    limit: limit ? parseInt(limit, 10) : null,
    techs: techs.length > 0 ? techs : null
  };

  const promise = discoverQuests(fetch, params).then(async (resp) => {
    if (resp.ok) {
      return (await resp.json()) as SearchQuestResult;
    } else {
      return { total: '0', isLastPage: true, quests: [] } as SearchQuestResult;
    }
  });
  return { questsPromise: promise };
}
