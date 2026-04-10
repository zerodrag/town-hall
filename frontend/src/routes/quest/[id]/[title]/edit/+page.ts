import { error } from '@sveltejs/kit';

export const load = async ({ parent }) => {
  const { quest, me } = await parent();
  // If not logged in
  if (!me) {
    error(401, 'Not logged in');
  }
  // If `me` doesn't own the quest
  if (me.userId !== quest.posterId) {
    if (quest.status === 'draft') {
      // Return Not Found if quest is a draft
      error(404, 'Quest ID not found');
    } else {
      // Return Forbidden otherwise
      error(403, 'You do not have access to this page');
    }
  }
  return { me };
};
