// 使用element-china-area-data库提供的中国省市区数据
import { regionData, codeToText } from 'element-china-area-data'

// 创建文本到编码的映射对象
const textToCode = {};

// 初始化textToCode映射
regionData.forEach(province => {
  textToCode[province.label] = province.value;
  if (province.children) {
    province.children.forEach(city => {
      textToCode[city.label] = city.value;
      if (city.children) {
        city.children.forEach(district => {
          textToCode[district.label] = district.value;
        });
      }
    });
  }
});

// 导出省市区数据，用于级联选择器
export const areaData = regionData

// 辅助函数：将级联选择器的值转换为字符串
export const formatAddress = (selectedOptions, detailAddress) => {
  if (!selectedOptions || selectedOptions.length === 0) {
    return detailAddress || '';
  }

  // 将编码转换为文本
  const addressParts = selectedOptions.map(code => codeToText[code]);
  if (detailAddress) {
    addressParts.push(detailAddress);
  }

  return addressParts.join(' ');
};

// 辅助函数：将地址字符串解析为级联选择器的值和详细地址
export const parseAddress = (address) => {
  if (!address) {
    return {
      selectedOptions: [],
      detailAddress: ''
    };
  }

  // 尝试解析已经是JSON格式的地址
  try {
    const addressObj = JSON.parse(address);
    if (addressObj && addressObj.selectedOptions && addressObj.detailAddress !== undefined) {
      return addressObj;
    }
  } catch (e) {
    // 不是JSON格式，继续处理
  }

  // 处理普通字符串格式的地址
  const parts = address.split(' ');

  // 如果地址部分少于3个，可能不是按省市区格式存储的
  if (parts.length < 3) {
    return {
      selectedOptions: [],
      detailAddress: address
    };
  }

  // 尝试匹配省市区
  const province = parts[0];
  const city = parts[1];
  const district = parts[2];
  const detailAddress = parts.slice(3).join(' ');

  // 尝试将文本转换为编码
  let provinceCode = textToCode[province];
  let cityCode, districtCode;

  if (provinceCode) {
    // 查找匹配的城市编码
    const provinceData = regionData.find(p => p.value === provinceCode);
    if (provinceData && provinceData.children) {
      const cityData = provinceData.children.find(c => c.label === city);
      if (cityData) {
        cityCode = cityData.value;

        // 查找匹配的区县编码
        if (cityData.children) {
          const districtData = cityData.children.find(d => d.label === district);
          if (districtData) {
            districtCode = districtData.value;
          }
        }
      }
    }
  }

  // 根据找到的编码构建选项数组
  const selectedOptions = [];
  if (provinceCode) selectedOptions.push(provinceCode);
  if (cityCode) selectedOptions.push(cityCode);
  if (districtCode) selectedOptions.push(districtCode);

  // 如果没有找到任何匹配的编码，则返回空数组和原始地址
  if (selectedOptions.length === 0) {
    return {
      selectedOptions: [],
      detailAddress: address
    };
  }

  return {
    selectedOptions,
    detailAddress
  };
};

// 将地址对象转换为JSON字符串存储
export const stringifyAddress = (selectedOptions, detailAddress) => {
  return JSON.stringify({
    selectedOptions,
    detailAddress: detailAddress || ''
  });
};

// 辅助函数：根据编码获取地址文本
export const getAddressText = (codes) => {
  if (!codes || !Array.isArray(codes) || codes.length === 0) {
    return '';
  }

  return codes.map(code => codeToText[code]).join(' ');
};