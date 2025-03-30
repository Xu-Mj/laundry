<template>
    <div class="post-list">
        <div class="fixed-bottom">
            <el-button type="primary" icon="Edit" circle class="publish-btn" @click="editorVisible = true" />
        </div>

         <el-dialog v-model="editorVisible" :close-on-press-escape="false" :close-on-click-modal="false" title="发布文章" width="80%">
            <el-form>
                <el-form-item label="文章标题" required>
                    <el-input v-model="postTitle" placeholder="请输入文章标题" maxlength="100" show-word-limit />
                </el-form-item>
                <el-form-item label="文章内容" required>
                    <QuillEditor v-model:content="editorContent" :min-height="500" />
                </el-form-item>
                <el-form-item label="标签">
                    <el-select
                        v-model="selectedTags"
                        multiple
                        filterable
                        allow-create
                        default-first-option
                        placeholder="请选择或创建标签"
                    >
                        <el-option v-for="tag in availableTags" :key="tag" :label="tag" :value="tag" />
                    </el-select>
                </el-form-item>
            </el-form>
            <template #footer>
                <el-button @click="handleCancelSubmitPost">取消</el-button>
                <el-button type="primary" @click="handleSubmit" :loading="submitting">发布</el-button>
            </template>
        </el-dialog>

        <div class="post-container">
            <div v-for="post in posts" :key="post.id" class="post-item">
                <div class="post-header">
                    <div class="post-title">{{ post.title }}</div>
                    <div class="post-meta">
                        <span class="post-time">{{ formatTime(post.createdAt) }}</span>
                        <div class="post-tags">
                            <el-tag v-for="tag in post.tags" :key="tag" type="info" size="small" class="tag-item">
                                {{ tag }}
                            </el-tag>
                        </div>
                    </div>
                </div>
                <div class="post-content" v-html="post.content"></div>

                <div class="interaction-area">
                    <div class="action-buttons">
                        <el-button type="text" :class="{ 'is-active': post.isLiked }" @click="handleLike(post)">
                            <el-icon>
                                <Star />
                            </el-icon>
                            <span class="action-text">{{ post.likeCount > 0 ? post.likeCount : '点赞' }}</span>
                        </el-button>

                        <el-button type="text" @click="handleCommentClick(post)">
                            <el-icon>
                                <ChatRound />
                            </el-icon>
                            <span class="action-text">{{ post.commentCount > 0 ? post.commentCount : '评论' }}</span>
                        </el-button>
                    </div>

                    <div class="comments" v-if="post.showComments">
                        <div class="main-comment-editor">
                            <el-input v-model="post.mainCommentContent" type="textarea" :rows="3"
                                placeholder="写下你的评论..." />
                            <div class="editor-actions">
                                <el-upload action="/api/upload" :on-success="handleMainImageSuccess"
                                    :show-file-list="false">
                                    <el-button link icon="Picture"></el-button>
                                </el-upload>
                                <el-button type="primary" @click="handleMainCommentSubmit(post)">发表评论</el-button>
                            </div>
                        </div>
                        <div class="comments-section" v-loading="post.loadingComments">
                            <div v-for="comment in post.comments" :key="comment.id" class="comment-card"
                                :class="{ 'parent-comment': !comment.parentCommentId }">
                                <div class="comment-header">
                                    <el-avatar :src="comment.author.avatar" class="comment-avatar" />
                                    <div class="comment-author-info">
                                        <span class="comment-author">{{ comment.author.storeName }}</span>
                                        <span class="comment-time">{{ formatTime(comment.createdAt) }}</span>
                                    </div>
                                </div>
                                <div class="comment-content">{{ comment.content }}</div>
                                <div class="comment-actions">
                                    <el-button type="text" size="small" :class="{ 'is-active': comment.isLiked }"
                                        @click="handleCommentLike(comment)">
                                        <el-icon>
                                            <Star />
                                        </el-icon>
                                        <span>{{ comment.likeCount }}</span>
                                    </el-button>
                                    <el-button type="text" size="small" @click="currentCommentId = comment.id">回复</el-button>
                                    <el-button type="danger" link size="small" v-if="comment.authorId === userStore.id"
                                        @click="handleDeleteComment(comment, post)">删除</el-button>
                                </div>

                                <!-- 回复编辑器 -->
                                <div class="reply-editor" v-if="currentCommentId === comment.id">
                                    <el-input v-model="commentContent" type="textarea" :rows="2" placeholder="回复评论..." />
                                    <div class="editor-actions">
                                        <el-upload action="/api/upload" :on-success="handleReplyImageSuccess"
                                            :show-file-list="false">
                                            <el-button link icon="Picture"></el-button>
                                        </el-upload>
                                        <div>
                                            <el-button size="small" @click="currentCommentId = null">取消</el-button>
                                            <el-button type="primary" size="small" @click="submitComment(comment)">提交</el-button>
                                        </div>
                                    </div>
                                </div>

                                <!-- 查看回复按钮 -->
                                <div v-if="comment.replyCount > 0 && !comment.showReplies" class="view-replies"
                                    @click="loadReplies(comment); comment.showReplies = true">
                                    <el-icon>
                                        <ArrowDown />
                                    </el-icon>
                                    查看 {{ comment.replyCount }} 条回复
                                </div>

                                <!-- 回复列表 -->
                                <div class="replies-container" v-if="comment.showReplies && comment.children">
                                    <div v-for="reply in comment.children" :key="reply.id" class="reply-card">
                                        <div class="comment-header">
                                            <el-avatar :src="reply.author.avatar" class="comment-avatar small-avatar" />
                                            <div class="comment-author-info">
                                                <span class="comment-author">{{ reply.author.storeName }}</span>
                                                <span v-if="reply.parentCommentId" class="reply-prefix">
                                                    回复 @{{ comment.author.storeName }}
                                                </span>
                                                <span class="comment-time">{{ formatTime(reply.createdAt) }}</span>
                                            </div>
                                        </div>
                                        <div class="comment-content">{{ reply.content }}</div>
                                        <div class="comment-actions">
                                            <el-button type="text" size="small" :class="{ 'is-active': reply.isLiked }"
                                                @click="handleCommentLike(reply)">
                                                <el-icon>
                                                    <Star />
                                                </el-icon>
                                                <span>{{ reply.likeCount }}</span>
                                            </el-button>
                                            <el-button type="danger" link size="small" v-if="reply.authorId === userStore.id"
                                                @click="handleDeleteComment(reply, post)">删除</el-button>
                                        </div>
                                    </div>

                                    <!-- 加载更多回复 -->
                                    <div v-if="comment.children.length < comment.replyCount" class="load-more-replies"
                                        @click="loadMoreReplies(comment)">
                                        <el-icon>
                                            <MoreFilled />
                                        </el-icon>
                                        加载更多回复
                                    </div>
                                </div>
                            </div>

                            <!-- 加载更多评论 -->
                            <div v-if="post.comments.length < post.commentCount && !post.loadingComments" class="load-more"
                                @click="loadMoreComments(post)">
                                <el-icon>
                                    <MoreFilled />
                                </el-icon>
                                加载更多评论
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- 无限滚动加载 -->
            <div v-if="posts.length < total && !loading" class="load-more-posts" @click="loadMorePosts">
                <el-icon>
                    <MoreFilled />
                </el-icon>
                加载更多文章
            </div>

            <!-- 加载中状态 -->
            <div v-if="loading" class="loading-container">
                <el-icon class="loading-icon">
                    <Loading />
                </el-icon>
                <span>加载中...</span>
            </div>
        </div>
    </div>
</template>

<script setup>
import { ref, onMounted, onBeforeUnmount, getCurrentInstance } from 'vue'
import { Star, Edit, ChatRound, ArrowDown, MoreFilled, Loading } from '@element-plus/icons-vue'
import { formatTime } from '@/utils'
import { listPosts, createPost, likePost, dislikePost } from '@/api/system/posts'
import { listComments, createComment, likeComment, dislikeComment } from '@/api/system/comment'
import { ElMessage, ElMessageBox } from 'element-plus'
import QuillEditor from '@/components/Editor/index.vue'
import useUserStore from '@/store/modules/user'

const { proxy } = getCurrentInstance();
const userStore = useUserStore()
const posts = ref([])
const editorVisible = ref(false)
const editorContent = ref('')
const postTitle = ref('')
const selectedTags = ref([])
const availableTags = ref(['技术', '分享', '问答', '讨论', '公告'])
const commentContent = ref('')
const currentCommentId = ref(0)
const loading = ref(false)
const submitting = ref(false)

// 分页参数
const pageNum = ref(1)
const pageSize = ref(10)
const total = ref(0)

const handleCancelSubmitPost = () => {
    // 提示用户是否保存草稿
    if (editorContent.value || postTitle.value) {
        ElMessageBox.confirm('是否保存为草稿？', '提示', {
            confirmButtonText: '保存',
            cancelButtonText: '不保存',
            type: 'warning'
        }).then(() => {
            // 保存草稿
            localStorage.setItem('postDraft', JSON.stringify({
                title: postTitle.value,
                content: editorContent.value,
                tags: selectedTags.value
            }))
            ElMessage.success('草稿已保存')
        }).finally(() => {
            resetEditor()
        })
    } else {
        resetEditor()
    }
}

// 重置编辑器
const resetEditor = () => {
    editorVisible.value = false
    editorContent.value = ''
    postTitle.value = ''
    selectedTags.value = []
}

const handleCommentClick = (post) => {
    post.showComments = !post.showComments;
    if (post.showComments && (!post.comments || post.comments.length === 0)) {
        loadComments(post, 1)
    }
}

const handleMainCommentSubmit = async (post) => {
    if (!post.mainCommentContent) {
        ElMessage.warning('请输入评论内容')
        return
    }
    try {
        const comment = await createComment({
            postId: post.id,
            authorId: userStore.id,
            content: post.mainCommentContent,
            parentCommentId: null
        });

        ElMessage.success('评论成功')
        post.comments.unshift(comment);
        post.mainCommentContent = ''
        post.commentCount++
    } catch (err) {
        console.error(err);
        proxy.notify.error('评论失败')
    }
}

// 处理图片上传成功
const handleMainImageSuccess = (res) => {
    // 处理图片上传逻辑
    if (res && res.url) {
        // 插入图片链接到评论内容中
        const imageUrl = res.url
        // 这里可以根据需要处理图片插入逻辑
    }
}

const handleReplyImageSuccess = (res) => {
    // 处理回复图片上传逻辑
    if (res && res.url) {
        // 插入图片链接到回复内容中
        const imageUrl = res.url
        // 这里可以根据需要处理图片插入逻辑
    }
}

// 自动保存间隔（30秒）
const AUTO_SAVE_INTERVAL = 30000
let saveTimer = null

// 组件挂载时恢复草稿
onMounted(() => {
    const draftJson = localStorage.getItem('postDraft')
    if (draftJson) {
        try {
            const draft = JSON.parse(draftJson)
            ElMessageBox.confirm('检测到未发布的草稿，是否恢复？', '提示', {
                confirmButtonText: '恢复',
                cancelButtonText: '放弃',
                type: 'warning'
            }).then(() => {
                postTitle.value = draft.title || ''
                editorContent.value = draft.content || ''
                selectedTags.value = draft.tags || []
                editorVisible.value = true
            }).catch(() => {
                localStorage.removeItem('postDraft')
            })
        } catch (e) {
            localStorage.removeItem('postDraft')
        }
    }

    // 启动自动保存定时器
    saveTimer = setInterval(() => {
        if (editorVisible.value && (editorContent.value || postTitle.value)) {
            localStorage.setItem('postDraft', JSON.stringify({
                title: postTitle.value,
                content: editorContent.value,
                tags: selectedTags.value
            }))
        }
    }, AUTO_SAVE_INTERVAL)

    // 添加窗口关闭提示
    window.addEventListener('beforeunload', handleBeforeUnload)
    
    // 加载文章列表
    loadPosts()
})

// 组件卸载时清理
onBeforeUnmount(() => {
    clearInterval(saveTimer)
    window.removeEventListener('beforeunload', handleBeforeUnload)
})

// 窗口关闭处理函数
const handleBeforeUnload = (e) => {
    if (editorVisible.value && (editorContent.value || postTitle.value)) {
        e.preventDefault()
        e.returnValue = '您有未保存的文章内容，确定要离开吗？'
    }
}

// 提交成功后清理本地存储
const handleSubmit = async () => {
    if (!postTitle.value) {
        ElMessage.warning('请输入文章标题')
        return
    }
    
    if (!editorContent.value) {
        ElMessage.warning('请输入文章内容')
        return
    }
    
    submitting.value = true
    try {
        await createPost({
            title: postTitle.value,
            content: editorContent.value,
            tags: selectedTags.value,
            authorId: userStore.id
        })
        localStorage.removeItem('postDraft')
        ElMessage.success('发布成功')
        resetEditor()
        
        // 刷新文章列表
        pageNum.value = 1
        loadPosts()
    } catch (err) {
        console.error(err)
        proxy.notify.error('发布失败')
    } finally {
        submitting.value = false
    }
}

const handleLike = async (post) => {
    try {
        if (post.isLiked) {
            await dislikePost(post.id)
            post.likeCount--
        } else {
            await likePost(post.id)
            post.likeCount++
        }
        post.isLiked = !post.isLiked
    } catch (err) {
        proxy.notify.error('点赞失败')
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
    } catch (err) {
        proxy.notify.error('点赞评论失败')
    }
}

// 删除评论
const handleDeleteComment = async (comment, post) => {
    try {
        await ElMessageBox.confirm('确定要删除这条评论吗？', '提示', {
            confirmButtonText: '确定',
            cancelButtonText: '取消',
            type: 'warning'
        })
        
        // 这里应该调用删除评论的API
        // await deleteComment(comment.id)
        
        // 从评论列表中移除
        const index = post.comments.findIndex(c => c.id === comment.id)
        if (index !== -1) {
            post.comments.splice(index, 1)
            post.commentCount--
            ElMessage.success('删除成功')
        }
    } catch (err) {
        if (err !== 'cancel') {
            proxy.notify.error('删除评论失败')
        }
    }
}

// 提交回复评论
const submitComment = async (parentComment) => {
    if (!commentContent.value) {
        ElMessage.warning('请输入评论内容')
        return
    }
    
    try {
        const comment = await createComment({
            postId: parentComment.postId,
            authorId: userStore.id,
            content: commentContent.value,
            parentCommentId: parentComment.id
        })
        
        // 如果父评论已经加载了回复，则添加到回复列表中
        if (parentComment.children) {
            parentComment.children.unshift(comment)
        } else {
            parentComment.children = [comment]
        }
        
        // 增加回复计数
        parentComment.replyCount++
        parentComment.showReplies = true
        
        // 清空输入框并关闭回复编辑器
        commentContent.value = ''
        currentCommentId.value = null
        
        ElMessage.success('回复成功')
    } catch (err) {
        console.error(err)
        proxy.notify.error('回复失败')
    }
}

// 加载评论
const loadComments = async (post, page = 1) => {
    if (post.loadingComments) return
    
    post.loadingComments = true
    try {
        const res = await listComments({
            postId: post.id,
            pageNum: page,
            pageSize: 10
        })
        
        if (page === 1) {
            post.comments = res.rows || []
        } else {
            post.comments = [...post.comments, ...(res.rows || [])]
        }
        
        post.commentCount = res.total
        post.commentPage = page
    } catch (err) {
        console.error(err)
        proxy.notify.error('加载评论失败')
    } finally {
        post.loadingComments = false
    }
}

// 加载更多评论
const loadMoreComments = (post) => {
    loadComments(post, (post.commentPage || 1) + 1)
}

// 加载回复
const loadReplies = async (comment) => {
    if (comment.loadingReplies) return
    
    comment.loadingReplies = true
    try {
        const res = await listComments({
            parentCommentId: comment.id,
            pageNum: 1,
            pageSize: 5
        })
        
        comment.children = res.rows || []
        comment.replyCount = res.total
        comment.replyPage = 1
    } catch (err) {
        console.error(err)
        proxy.notify.error('加载回复失败')
    } finally {
        comment.loadingReplies = false
    }
}

// 加载更多回复
const loadMoreReplies = async (comment) => {
    if (comment.loadingReplies) return
    
    comment.loadingReplies = true
    try {
        const res = await listComments({
            parentCommentId: comment.id,
            pageNum: (comment.replyPage || 1) + 1,
            pageSize: 5
        })
        
        comment.children = [...comment.children, ...(res.rows || [])]
        comment.replyPage++
    } catch (err) {
        console.error(err)
        proxy.notify.error('加载更多回复失败')
    } finally {
        comment.loadingReplies = false
    }
}

// 加载文章列表
const loadPosts = async () => {
    loading.value = true
    try {
        const res = await listPosts({
            pageNum: pageNum.value,
            pageSize: pageSize.value
        })
        
        posts.value = res.rows.map(post => ({
            ...post,
            showComments: false,
            comments: [],
            mainCommentContent: '',
            loadingComments: false
        }))
        
        total.value = res.total
    } catch (err) {
        console.error(err)
        proxy.notify.error('加载文章失败')
    } finally {
        loading.value = false
    }
}

// 加载更多文章
const loadMorePosts = () => {
    pageNum.value++
    loading.value = true
    
    listPosts({
        pageNum: pageNum.value,
        pageSize: pageSize.value
    }).then(res => {
        posts.value = [...posts.value, ...res.rows.map(post => ({
            ...post,
            showComments: false,
            comments: [],
            mainCommentContent: '',
            loadingComments: false
        }))]
        
        total.value = res.total
    }).catch(err => {
        console.error(err)
        proxy.notify.error('加载更多文章失败')
        pageNum.value--
    }).finally(() => {
        loading.value = false
    })
}
</script>

<style scoped>
.post-list {
    max-width: 900px;
    margin: 0 auto;
    padding: 20px;
}

.fixed-bottom {
    position: fixed;
    bottom: 30px;
    right: 30px;
    z-index: 100;
}

.publish-btn {
    width: 60px;
    height: 60px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    transition: all 0.3s ease;
}

.publish-btn:hover {
    transform: scale(1.1);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
}

.post-container {
    display: flex;
    flex-direction: column;
    gap: 24px;
}

.post-item {
    background-color: var(--el-bg-color);
    border-radius: 12px;
    box-shadow: var(--el-box-shadow-lighter);
    overflow: hidden;
    transition: all 0.3s ease;
    padding: 24px;
}

.post-item:hover {
    box-shadow: var(--el-box-shadow-light);
    transform: translateY(-2px);
}

.post-header {
    margin-bottom: 16px;
}

.post-title {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 8px;
}

.post-meta {
    display: flex;
    align-items: center;
    justify-content: space-between;
    color: #909399;
    font-size: 0.9rem;
}

.post-tags {
    display: flex;
    gap: 8px;
}

.tag-item {
    margin-right: 0;
}

.post-content {
    margin-bottom: 20px;
    line-height: 1.6;
    color: #606266;
    overflow-wrap: break-word;
}

.interaction-area {
    border-top: 1px solid #f0f0f0;
    padding-top: 16px;
}

.action-buttons {
    display: flex;
    gap: 24px;
}

.action-buttons .el-button {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #606266;
    transition: all 0.2s ease;
}

.action-buttons .el-button:hover,
.action-buttons .el-button.is-active {
    color: var(--el-color-primary);
}

.action-text {
    font-size: 0.9rem;
}

.comments {
    margin-top: 16px;
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 16px;
    animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}

.main-comment-editor {
    margin-bottom: 20px;
}

.editor-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 8px;
}

.comments-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
}

.comment-card {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 16px;
    box-shadow: var(--el-box-shadow-lighter);
    transition: all 0.2s ease;
}

.comment-card:hover {
    box-shadow: var(--el-box-shadow-light);
}

.parent-comment {
    border-left: 3px solid var(--el-color-primary);
}

.comment-header {
    display: flex;
    align-items: center;
    margin-bottom: 8px;
}

.comment-avatar {
    margin-right: 12px;
    border: 2px solid #f0f0f0;
}

.small-avatar {
    width: 28px;
    height: 28px;
}

.comment-author-info {
    display: flex;
    flex-direction: column;
}

.comment-author {
    font-weight: 600;
    color: #303133;
    font-size: 0.95rem;
}

.comment-time {
    color: #909399;
    font-size: 0.8rem;
    margin-top: 2px;
}

.reply-prefix {
    color: #909399;
    font-size: 0.85rem;
    margin-top: 2px;
}

.comment-content {
    margin: 8px 0;
    color: #606266;
    line-height: 1.5;
    word-break: break-word;
}

.comment-actions {
    display: flex;
    gap: 16px;
    margin-top: 8px;
}

.reply-editor {
    margin-top: 12px;
    padding: 12px;
    background-color: var(--el-fill-color);
    border-radius: 6px;
    animation: fadeIn 0.3s ease;
}

.replies-container {
    margin-top: 12px;
    padding-left: 12px;
    display: flex;
    flex-direction: column;
    gap: 12px;
    animation: fadeIn 0.3s ease;
}

.reply-card {
    background-color: #f5f7fa;
    border-radius: 6px;
    padding: 12px;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.03);
}

.view-replies, 
.load-more-replies,
.load-more,
.load-more-posts {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 6px;
    padding: 8px;
    color: var(--el-color-primary);
    cursor: pointer;
    font-size: 0.9rem;
    transition: all 0.2s ease;
    border-radius: 6px;
    margin-top: 8px;
}

.view-replies:hover, 
.load-more-replies:hover,
.load-more:hover,
.load-more-posts:hover {
    background-color: rgba(var(--el-color-primary-rgb), 0.1);
}

.load-more-posts {
    padding: 12px;
    background-color: #f5f7fa;
    border-radius: 8px;
    margin-top: 16px;
}

.loading-container {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 20px;
    color: #909399;
}

.loading-icon {
    animation: rotate 1s linear infinite;
}

@keyframes rotate {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

/* 响应式设计 */
@media (max-width: 768px) {
    .post-list {
        padding: 12px;
    }
    
    .post-item {
        padding: 16px;
    }
    
    .post-title {
        font-size: 1.3rem;
    }
    
    .fixed-bottom {
        bottom: 20px;
        right: 20px;
    }
    
    .publish-btn {
        width: 50px;
        height: 50px;
    }
}
</style>
