import invoke from '@/utils/invoke'

// 更新用户的引导记录
export function updateTourGuide(pageKey) {
  return invoke('update_tour_guide', { pageKey })
}

// 检查用户是否已完成特定页面的引导
export function checkTourCompleted(pageKey) {
  return invoke('check_tour_completed', { pageKey })
}