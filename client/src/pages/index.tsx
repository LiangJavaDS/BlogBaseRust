import styles from './index.less';
import React, { useCallback } from 'react';
import ArticleCatalogue from "@/pages/article/articleCatalogue/index";
import { Button } from 'antd';
import { history } from 'umi';

export default function IndexPage() {

  /** 跳转到博客详情页 */
  const goToAddUser = useCallback(() => {
    history.push({
      pathname: '/addUser',
    });
  }, [])

  return (
    <div>
      <ArticleCatalogue />
      <Button onClick={goToAddUser}>新增用户</Button>
    </div>
  );
}
