<template>
    <div class="post-list">
        <div class="fixed-bottom">
            <el-button type="primary" icon="Edit" circle class="publish-btn" @click="editorVisible = true" />
        </div>

        <el-dialog v-model="editorVisible" :close-on-press-escape="false" :close-on-click-modal="false" title="发布文章" width="80%">
            <QuillEditor v-model:content="editorContent" :min-height="500"  />
            <template #footer>
                <el-button @click="handleCancelSubmitPost">取消</el-button>
                <el-button type="primary" @click="handleSubmit">发布</el-button>
            </template>
        </el-dialog>

        <div class="post-container">
            <div v-for="post in posts" :key="post.id" class="post-item">
                <div class="post-header">
                    <div class="post-title">{{ post.title }}</div>
                    <div class="post-time">{{ formatTime(post.createdAt) }}</div>
                </div>
                <div class="post-content">{{ post.content }}</div>

                <div class="post-tags">
                    <el-tag v-for="tag in post.tags" :key="tag" type="info" size="small" class="tag-item">
                        {{ tag }}
                    </el-tag>
                </div>

                <div class="interaction-area">
                    <div class="action-icons" style="display: flex; justify-content: space-around;">
                        <el-button type="text" @click="handleLike(post)">
                            <el-icon>
                                <Star />
                            </el-icon>
                            {{ post.likeCount }}
                        </el-button>

                        <el-button type="text" @click="handleCommentClick(post)"><el-icon>
                                <ChatRound />
                            </el-icon>
                            {{ post.commentCount > 0 ? post.commentCount : '' }}
                        </el-button>
                    </div>

                    <div class="comments" v-if="post.showComments">
                        <div class="main-comment-editor">
                            <h3>评论</h3>
                            <el-input v-model="post.mainCommentContent" type="textarea" :rows="3"
                                placeholder="请输入评论内容" />
                            <div class="editor-actions"
                                style="display: flex; justify-content: flex-end; gap: 1rem; margin-top: 1rem;">
                                <el-upload action="/api/upload" :on-success="handleMainImageSuccess"
                                    :show-file-list="false">
                                    <el-button link size="bigger" icon="Picture"></el-button>
                                </el-upload>
                                <el-button type="primary" @click="handleMainCommentSubmit(post)">提交评论</el-button>
                            </div>
                        </div>
                        <div class="comments-section" v-loading="post.loadingComments">

                            <div v-for="comment in post.comments" :key="comment.id" class="comment-item"
                                :style="{ marginLeft: comment.parentCommentId ? '20px' : '0' }">
                                <el-avatar :src="comment.author.avatar" class="comment-avatar" />
                                <div class="comment-body">
                                    <div class="comment-header">
                                        <span class="comment-author">{{ comment.author.storeName }}</span>
                                    </div>
                                    <div class="comment-content">{{ comment.content }}</div>
                                    <div class="comment-meta">
                                        <span class="comment-time">{{ formatTime(comment.createdAt) }}</span>
                                        <el-button type="text" size="small" @click="handleCommentLike(comment)">
                                            <el-icon>
                                                <Star />
                                            </el-icon> {{ comment.likeCount }}
                                        </el-button>
                                        <el-button type="text" size="small"
                                            @click="currentCommentId = comment.id">回复</el-button>
                                        <el-button type="danger" link size="small"
                                            v-if="comment.authorId === useUserStore.id" @click="">删除</el-button>
                                    </div>
                                    <div v-if="comment.replyCount > 0" class="view-replies"
                                        @click="loadReplies(comment)">
                                        查看 {{ comment.replyCount }} 条回复
                                    </div>
                                    <div class="sub-comments" v-if="comment.children">
                                        <div v-for="child in comment.children" :key="child.id"
                                            class="comment-item sub-comment-item" style="margin-left: 40px;">
                                            <el-avatar :src="child.author.avatar" class="comment-avatar small-avatar" />
                                            <div class="comment-body">
                                                <div class="sub-comment-header">
                                                    <span class="comment-author">{{ child.author.storeName }}</span>
                                                    <span v-if="child.parentCommentId" class="reply-prefix">
                                                        回复@{{ comment.author.storeName }}
                                                    </span>
                                                </div>
                                                <div class="comment-content sub-comment-content">{{ child.content }}
                                                </div>
                                                <div class="comment-meta compact-meta">
                                                    <span class="comment-time">{{ formatTime(child.createdAt) }}</span>
                                                    <el-button type="text" size="small"
                                                        @click="handleCommentLike(child)">
                                                        <el-icon>
                                                            <Star />
                                                        </el-icon> {{ child.likeCount }}
                                                    </el-button>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                <div class="reply-editor" v-if="currentCommentId === comment.id"
                                    :style="{ marginLeft: comment.parentCommentId ? '40px' : '20px' }">
                                    <el-input v-model="commentContent" type="textarea" :rows="2"
                                        placeholder="请输入回复内容" />
                                    <el-upload action="/api/upload" :on-success="handleReplyImageSuccess"
                                        :show-file-list="false">
                                        <el-button type="primary" icon="Picture" size="small">添加图片</el-button>
                                    </el-upload>
                                    <div class="editor-actions">
                                        <el-button size="small" @click="currentCommentId = null">取消</el-button>
                                        <el-button type="primary" size="small"
                                            @click="submitComment(comment)">提交</el-button>
                                    </div>
                                </div>

                            </div>
                        </div>
                    </div>
                </div>
            </div>
            <div style="display: flex; justify-content: center; align-items: center;">
                <el-pagination v-model:current-page="postPage" v-model:page-size="pageSize" :page-sizes="[10, 20, 50]"    
                layout="total, sizes, prev, pager, next" :total="total" @current-change="handlePageChange"
                @size-change="handleSizeChange" />
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { Star, Edit } from '@element-plus/icons-vue'
import { formatTime } from '@/utils'
import { listPosts, createPost, likePost, dislikePost } from '@/api/system/posts'
import { listComments, createComment, likeComment, dislikeComment } from '@/api/system/comment'
import { ElMessage } from 'element-plus'
import QuillEditor from '@/components/Editor/index.vue'
import useUserStore from '@/store/modules/user'

const userStore = useUserStore()
const posts = ref([])
const editorVisible = ref(false)
const editorContent = ref('')
const commentContent = ref('')
const commentVisible = ref(false)
const currentPost = ref(null)
const currentCommentId = ref(0)

const postPage = ref(1)
const pageSize = ref(10)
const total = ref(0)

const handleCancelSubmitPost = () => {
    editorVisible.value = false
    editorContent.value = ''
}
const handleCommentClick = (post) => {
    post.showComments = !post.showComments;
    if (post.showComments) {
        loadComments(post, post.commentPage)
        post.commentPage += 1
    } else {
        post.commentPage = 1
    }
}

const handleMainCommentSubmit = async (post) => {
    if (!post.mainCommentContent) {
        ElMessage.warning('请输入评论内容')
        return
    }
    try {
        console.log(userStore.id)
        const comment = await createComment({
            postId: post.id,
            authorId: userStore.id,
            content: post.mainCommentContent,
            parentCommentId: null
        });

        ElMessage.success('评论成功')
        post.comments.unshift(comment);
        post.mainCommentContent = ''
        post.commentTotal++
        console.log(post)
    } catch (err) {
        console.error(err);
        ElMessage.error('评论失败')
    } finally {

    }
}

// 自动保存间隔（30秒）
const AUTO_SAVE_INTERVAL = 30000
let saveTimer = null

// 组件挂载时恢复草稿
onMounted(() => {
  const draft = localStorage.getItem('postDraft')
  if (draft) {
    ElMessageBox.confirm('检测到未发布的草稿，是否恢复？', '提示', {
      confirmButtonText: '恢复',
      cancelButtonText: '放弃',
      type: 'warning'
    }).then(() => {
      editorContent.value = draft
      editorVisible.value = true
    }).finally(() => {
      localStorage.removeItem('postDraft')
    })
  }
  
  // 启动自动保存定时器
  saveTimer = setInterval(() => {
    if (editorContent.value) {
      localStorage.setItem('postDraft', editorContent.value)
    }
  }, AUTO_SAVE_INTERVAL)

  // 添加窗口关闭提示
  window.addEventListener('beforeunload', handleBeforeUnload)
})

// 组件卸载时清理
onBeforeUnmount(() => {
  clearInterval(saveTimer)
  window.removeEventListener('beforeunload', handleBeforeUnload)
})

// 窗口关闭处理函数
const handleBeforeUnload = (e) => {
  if (editorContent.value) {
    e.preventDefault()
    e.returnValue = '您有未保存的文章内容，确定要离开吗？'
  }
}

// 提交成功后清理本地存储
const handleSubmit = async () => {
  try {
    await createPost({
      title: '未命名文章',
      content: editorContent.value,
      authorId: userStore.id
    })
    localStorage.removeItem('postDraft')
    const post = await createPost({
        content: editorContent.value,
        author: '当前用户'
    });
    ElMessage.success('发布成功')
    editorVisible.value = false
    posts.value.unshift(post)
  } catch (err) {
      ElMessage.error('发布失败')
  }
}


const handleLike = async (post) => {
    try {
        await likePost(post.post.id)
        const index = posts.value.findIndex(p => p.post.id === post.post.id)
        if (index > -1) posts.value[index].post.likeCount++
    } catch (err) {
        ElMessage.error('点赞失败')
    }
}

const handleCommentLike = async (comment) => {
    try {
        if (comment.isLiked) {
            await dislikeComment(comment.id)
            comment.likeCount--
        } else {
            await likeComment(comment.id)
            comment.likeCount++
        }
        comment.isLiked = !comment.isLiked
        const postIndex = posts.value.findIndex(p => p.post.id === comment.postId)
        if (postIndex > -1) {
            const commentIndex = posts.value[postIndex].comments.findIndex(c => c.id === comment.id)
            posts.value[postIndex].comments[commentIndex].likeCount++
        }
    } catch (err) {
        ElMessage.error('评论点赞失败')
    }
    try {
        await likePost(postId)
        const index = posts.value.findIndex(p => p.post.id === postId)
        if (index > -1) posts.value[index].post.likeCount++
    } catch (err) {
        ElMessage.error('点赞失败')
    }
}

const loadComments = async (post, page = 1) => {
    try {
        post.loadingComments = true;
        const res = await listComments({
            pageNum: page,
            pageSize: 5,
            postId: post.id,
            parentCommentId: null
        });
        if (page === 1) {
            post.comments = res.rows.map(c => ({ ...c, children: [] }));
        } else {
            post.comments = [...post.comments, ...res.rows.map(c => ({ ...c, children: [] }))];
        }
        post.commentCount = res.total;
        post.commentPage = page;
    } catch (err) {
        console.error(err)
        ElMessage.error('加载评论失败');
    } finally {
        post.loadingComments = false;
    }
};

const loadReplies = async (comment, page = 1) => {
    try {
        comment.loadingReplies = true;
        const res = await listComments(comment.postId, {
            page: page,
            pageSize: 5,
            parentCommentId: comment.id
        });

        if (!comment.children) comment.children = [];
        comment.children = [...comment.children, ...res.rows];
        comment.replyPage = page;
    } catch (err) {
        ElMessage.error('加载回复失败');
    } finally {
        comment.loadingReplies = false;
    }
};

const handleCommentPageChange = (post, newPage) => {
    post.commentPage = newPage;
    loadComments(post);
};

const submitComment = async (comment) => {
    try {
        const res = await createComment({
            postId: comment.postId,
            content: commentContent.value,
            parentCommentId: currentCommentId.value === comment.id ? comment.id : null
        });
        ElMessage.success('评论成功')
        comment.children.push(res)
        comment.replyCount++
        commentContent.value = ''
    } catch (err) {
        ElMessage.error('评论失败')
    }
}


const handlePageChange = (newPage) => {
    postPage.value = newPage
    loadPosts()
}

const handleSizeChange = (newSize) => {
    pageSize.value = newSize
    loadPosts()
}

const loadPosts = async () => {
    try {
        const res = await listPosts({
            pageNum: postPage.value,
            pageSize: pageSize.value
        })
        posts.value = res.rows;
        total.value = res.total
    } catch (err) {
        ElMessage.error('获取数据失败')
    }
}


onMounted(async () => {
    loadPosts()
})
</script>

<style lang="scss" scoped>
.post-list {
    padding: 1rem;
}
.fixed-bottom {
    position: fixed;
    bottom: 40px;
    right: 40px;
    z-index: 999;

    .publish-btn {
        width: 60px;
        height: 60px;
        font-size: 24px;
        box-shadow: var(--el-box-shadow);
    }
}

.post-container {
    display: flex;
    flex-direction: column;
    justify-content: flex-start;
    align-items: center;
    gap: 1rem;
    overflow-y: auto;
    .post-item {
        // background: #fff;
        border-radius: 8px;
        max-width: 800px;
        padding: 20px;
        box-shadow: var(--el-box-shadow);

        .post-header {
            //display: flex;
            //justify-content: space-between;
            margin-bottom: 10px;
            color: #666;
        }

        .post-content {
            font-size: 16px;
            line-height: 1.6;
            margin-bottom: 15px;

            :deep(img) {
                max-width: 100%;
                height: auto;
            }
        }

        .interaction-area {
            margin-top: 1rem;
            border-top: 1px solid #eee;
            padding-top: 10px;

            .comments-section {
                margin-top: 10px;

                .comment-item {
                    padding: 8px 0;
                    border-bottom: 1px solid #eee;
                    font-size: 14px;
                    color: #666;
                }
            }
        }
    }
}

.post-title {
    font-size: 18px;
    font-weight: 600;
    margin: 10px 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.comment-item {
    margin-left: calc(20px * var(--level));
}

.comment-item {
    display: flex;
    margin-bottom: 16px;
    padding: 12px;
    // background: #f8f9fa;
    // border-radius: 8px;
}

.comment-avatar {
    margin-right: 12px;
    flex-shrink: 0;
}

.comment-body {
    flex: 1;
}

.comment-author {
    font-weight: 500;
    color: #1f2d3d;
    margin-right: 8px;
}

.comment-content {
    margin: 8px 0;
    color: #5e6d82;
    line-height: 1.6;
}

.comment-meta {
    display: flex;
    align-items: center;
    gap: 16px;
    font-size: 12px;
    color: #99a9bf;
}

.view-replies {
    color: #409eff;
    cursor: pointer;
    font-size: 12px;
    margin-top: 8px;
}

.view-replies:hover {
    text-decoration: underline;
}
</style>
