import { error, redirect } from '@sveltejs/kit';
import { resolve } from '$app/paths';
import type { GetUserQuestParams, GetUserQuestResult, User } from '$lib/backend/generated-types.js';
import { getQuestFromUser } from '$lib/backend/quest';
import { getUser } from '$lib/backend/user.js';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, url, params, route }) => {
  const result = await getUser(fetch, params.id);
  if (!result.ok) error(result.status, await result.text());
  const user: User = await result.json();
  if (user.handle !== params.handle) {
    redirect(
      308,
      resolve(route.id, {
        ...params,
        handle: user.handle
      })
    );
  }
  const page = url.searchParams.get('page');
  const limit = url.searchParams.get('limit');
  const getQuestParams: GetUserQuestParams = {
    page: page ? parseInt(page, 10) : null,
    limit: limit ? parseInt(limit, 10) : null
  };
  const questsPromise = getQuestFromUser(fetch, user.userId, getQuestParams).then(async (resp) => {
    if (resp.ok) {
      return (await resp.json()) as GetUserQuestResult;
    } else {
      return { total: '0', isLastPage: true, quests: [] } as GetUserQuestResult;
    }
  });

  return { user, questsPromise };
};
