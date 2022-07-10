import React, { FC, useCallback, useMemo } from 'react';
import ArticleCatalogue from "@/pages/article/articleList/index";
import { history } from 'umi';
import { RouteProps } from 'umi/node_modules/@types/react-router';
import { connect } from 'dva';
import ArticleDetail from '@/pages/article/articleDetail/index';
import ArticleEditedForm from '@/pages/article/articleEditedForm/index';
import UserForm from '@/pages/user/userForm';
import { Header, Body, Footer, AntdButton, BlogName, FooterLink } from './styles';
import LoginForm from "@/pages/login/index";

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

    /** 跳转至登录页面 */
    const login = useCallback(() => {
        history.push({ pathname: '/login' });
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
            case '/login':
                return <LoginForm />
            default:
                return <ArticleCatalogue />
        }
    }, [route.path])

    return (
        <>
            <Header>
                {/* <Button onClick={goToAddUser}>新增用户</Button> */}
                <BlogName>
                    {/* 梁凉凉 Blog */}
                </BlogName>
                <div>
                    <AntdButton onClick={addBlog}>新增博客</AntdButton>
                    <AntdButton onClick={login}>登录</AntdButton>
                </div>
            </Header>
            <Body>
                {bodyComponent}
            </Body>
            <Footer>
                <FooterLink onClick={() => window.open("https://beian.miit.gov.cn")}>
                    鲁ICP备2022022109号 © 2022 HY Liang
                </FooterLink >
            </Footer>
        </>
    );
}

export default connect((user: any) => ({ user }))(MatchPath);