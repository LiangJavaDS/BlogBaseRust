import { history } from 'umi';

/** 跳转到详情页 */
export function goToArticleDetail(id: string) {
    history.push({
        pathname: '/articleDetail',
        query: {
            blogId: id
        }
    });
}