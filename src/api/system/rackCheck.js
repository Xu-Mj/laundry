import { listRack } from '@/api/system/rack';

/**
 * 检查衣挂数据是否已初始化
 * @returns {Promise<boolean>} 返回一个Promise，解析为布尔值，表示衣挂数据是否已初始化
 */
export function checkRackInitialized() {
  return new Promise((resolve) => {
    listRack().then(response => {
      // 如果衣挂列表为空或长度为0，则表示未初始化
      const isInitialized = response && response.length > 0;
      resolve(isInitialized);
    }).catch(() => {
      // 发生错误时，默认为未初始化
      resolve(false);
    });
  });
}