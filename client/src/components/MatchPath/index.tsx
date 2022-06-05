import React, { FC, useCallback, useMemo } from 'react';
import ArticleCatalogue from "@/pages/article/articleCatalogue/index";
import { Button } from 'antd';
import { history } from 'umi';
import { RouteProps } from 'umi/node_modules/@types/react-router';
import { connect } from 'dva';
import styles from './index.less';
import ArticleDetail from '@/pages/article/articleDetail/index';
import ArticleEditedForm from '@/pages/article/articleEditedForm/index';
import UserForm from '@/pages/user/userForm';

type MatchPathProps = {
    route: RouteProps;
}
const MatchPath: FC<MatchPathProps> = ({ route }) => {

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

    /** 将路由转为 */
    const bodyComponent = useMemo(() => {
        switch (route.path) {
            case '/articleDetail':
                return <ArticleDetail />
            case '/articleEditedForm':
                return <ArticleEditedForm />
            case '/addUser':
                return <UserForm />
            default:
                return <ArticleCatalogue />
        }
    }, [route.path])

    return (
        <>
            <div className={styles.header}>
                我是头部
                <Button onClick={goToAddUser}>新增用户</Button>
                <Button onClick={addBlog}>新增博客</Button>
            </div>
            <div className={styles.main}>
                {bodyComponent}
            </div>
            <div className={styles.foot}>
                我是底部
            </div>
        </>
    );
}

export default connect((user) => ({ user }))(MatchPath);