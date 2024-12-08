import invoke from '@/utils/invoke'

// 查询支出列表
export function listExpenditure(query) {
  const pageParams = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const exp = {
    orderId: query.orderId,
    clothIds: query.clothIds,
    expTitle: query.expTitle,
    recvAccount: query.recvAccount,
    recvAccountTitle: query.recvAccountTitle,
    expType: query.expType,
  };
  return invoke('get_exp_pagination', { pageParams, exp })
}

// 查询支出详细
export function getExpenditure(expId) {
  return invoke('get_exp_by_id', { expId })
}

// 新增支出
export function addExpenditure(data) {
  return invoke('create_exp', { exp: data })
}

// 修改支出
export function updateExpenditure(data) {
  return invoke('update_exp', { exp: data })

}

// 删除支出
export function delExpenditure(expId) {
  return invoke('delete_exp', { ids: [].concat(expId) })
}
