import request from '@/utils/request'

// 查询文章列表
export function listComments(query) {
  const params = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
  const comment = { postId: query.postId, parentCommentId: query.parentCommentId };

  return request({
    url: '/comments',
    method: 'get',
    params: { comment, params }
  })
}

// 创建文章
export function createComment(data) {
  return request({
    url: '/comments',
    method: 'post',
    data: data
  })
}

// 更新文章
export function likeComment(commentId) {
  return request({
    url: '/comments/like/' + commentId,
    method: 'put',
  })
}

export function dislikeComment(commentId) {
  return request({
    url: '/comments/unlike/' + commentId,
    method: 'put',
  })
}

// 删除文章
export function deletePosts(ids) {
  return request({
    url: '/posts',
    method: 'delete',
    params: { ids: [].concat(ids) }
  })
}

// 获取文章详情
export function getPost(id) {
  return request({
    url: `/posts/${id}`,
    method: 'get'
  })
}