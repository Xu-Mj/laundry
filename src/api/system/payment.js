import invoke from '@/utils/invoke'

export function getTotalAmountAndAvgConsume(userId) {
  return invoke('get_total_amount', { userId })
}
