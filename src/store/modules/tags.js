import { defineStore } from 'pinia'
import { listTagsNoLimit, addTags } from "@/api/system/tags";

const useTagsStore = defineStore('tags', {
  state: () => ({
    colorList: [], // 颜色标签 'Color'
    flawList: [], // 瑕疵标签 'PreCleaningFlaws'
    estimateList: [], // 预估标签 'PostCleaningProjection'
    brandList: [], // 品牌标签 'Brand'
    isInitialized: false
  }),
  getters: {
    getColorList: (state) => state.colorList,
    getFlawList: (state) => state.flawList,
    getEstimateList: (state) => state.estimateList,
    getBrandList: (state) => state.brandList
  },
  actions: {
    // 初始化所有标签数据
    async initTags() {
      if (this.isInitialized) return;
      
      const promises = [];
      
      // 获取颜色列表
      const colorPromise = listTagsNoLimit({ tagOrder: 'Color', status: "0" }).then(response => {
        this.colorList = response;
      });
      promises.push(colorPromise);
      
      // 获取瑕疵列表
      const flawPromise = listTagsNoLimit({ tagOrder: 'PreCleaningFlaws', status: "0" }).then(response => {
        this.flawList = response;
      });
      promises.push(flawPromise);
      
      // 获取预估列表
      const estimatePromise = listTagsNoLimit({ tagOrder: 'PostCleaningProjection', status: "0" }).then(response => {
        this.estimateList = response;
      });
      promises.push(estimatePromise);
      
      // 获取品牌列表
      const brandPromise = listTagsNoLimit({ tagOrder: 'Brand', status: "0" }).then(response => {
        this.brandList = response;
      });
      promises.push(brandPromise);
      
      // 等待所有异步操作完成
      await Promise.all(promises);
      console.log('所有标签数据已初始化', this);
      
      this.isInitialized = true;
    },
    
    // 根据标签类型获取标签列表
    getTagsByOrder(tagOrder) {
      switch (tagOrder) {
        case 'PreCleaningFlaws':
          return this.flawList;
        case 'PostCleaningProjection':
          return this.estimateList;
        case 'Color':
          return this.colorList;
        case 'Brand':
          return this.brandList;
        default:
          return [];
      }
    },
    
    // 添加新标签并更新缓存
    async addTag(tagOrder, tagName) {
      try {
        const newTag = await addTags({ tagName, tagOrder });
        
        // 根据标签类型将新标签添加到相应的列表中
        switch (tagOrder) {
          case 'PreCleaningFlaws':
            this.flawList.push(newTag);
            break;
          case 'PostCleaningProjection':
            this.estimateList.push(newTag);
            break;
          case 'Color':
            this.colorList.push(newTag);
            break;
          case 'Brand':
            this.brandList.push(newTag);
            break;
        }
        
        return newTag;
      } catch (error) {
        console.error('添加标签失败:', error);
        throw error;
      }
    },
    
    // 强制刷新特定类型的标签数据
    async refreshTagsByType(tagOrder) {
      try {
        const response = await listTagsNoLimit({ tagOrder, status: "0" });
        
        switch (tagOrder) {
          case 'PreCleaningFlaws':
            this.flawList = response;
            break;
          case 'PostCleaningProjection':
            this.estimateList = response;
            break;
          case 'Color':
            this.colorList = response;
            break;
          case 'Brand':
            this.brandList = response;
            break;
        }
        
        return response;
      } catch (error) {
        console.error(`刷新${tagOrder}标签数据失败:`, error);
        throw error;
      }
    },
    
    // 刷新所有类型的标签数据
    async refreshTags() {
      try {
        this.resetCache();
        await this.initTags();
        console.log('所有标签数据已刷新');
      } catch (error) {
        console.error('刷新所有标签数据失败:', error);
        throw error;
      }
    },
    
    // 重置所有标签数据的缓存状态
    resetCache() {
      this.isInitialized = false;
      this.colorList = [];
      this.flawList = [];
      this.estimateList = [];
      this.brandList = [];
    }
  },
  persist: {
    enabled: true,
    strategies: [
      {
        key: 'tags_cache',
        storage: localStorage
      }
    ]
  }
});

export default useTagsStore; 