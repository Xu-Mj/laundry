import request from '@/utils/request'

// 查询文章列表
export function listPosts(query) {
    const params = { pageSize: query.pageSize, page: query.pageNum, params: query.params };
    const post = { title: query.title, author: query.author, status: query.status };

    return request({
        url: '/posts',
        method: 'get',
        params: { post, params }
    })
}

// 创建文章
export function createPost(data) {
    return request({
        url: '/posts',
        method: 'post',
        data: data
    })
}

// 更新文章
export function updatePost(data) {
    return request({
        url: '/posts',
        method: 'put',
        data: data
    })
}

export function likePost(postId) {
    return request({
        url: '/posts/like/' + postId,
        method: 'put',
    })
}

export function dislikePost(postId) {
    return request({
        url: '/posts/unlike/' + postId,
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