import styles from './index.less';
import React, { useCallback } from 'react';
import ArticleCatalogue from "@/pages/article/articleCatalogue/index";
import { Button } from 'antd';
import { history } from 'umi';

export default function IndexPage(props) {

  /** 跳转到博客详情页 */
  const goToAddUser = useCallback(() => {
    history.push({
      pathname: '/addUser',
    });
  }, [])

  /** 跳转至新增页面 */
  const addBlog = useCallback(() => {
    history.push({ pathname: '/articleEditedForm' });
  }, [])

  console.log('7878props', props);


  return (
    <>
      <div className={styles.header}>
        我是头部
        <Button onClick={goToAddUser}>新增用户</Button>
        <Button onClick={addBlog}>新增博客</Button>
      </div>
      <div className={styles.main}>
        <ArticleCatalogue />
      </div>
      <div className={styles.foot}>
        我是底部
      </div>
    </>
  );
}
