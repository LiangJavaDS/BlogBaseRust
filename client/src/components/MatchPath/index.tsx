import React, { FC, useCallback, useMemo } from 'react';
import ArticleCatalogue from "@/pages/article/articleList/index";
import { Dispatch, history, UserModelState } from 'umi';
import { RouteProps } from 'umi/node_modules/@types/react-router';
import { connect } from 'dva';
import ArticleDetail from '@/pages/article/articleDetail/index';
import ArticleEditedForm from '@/pages/article/articleEditedForm/index';
import UserForm from '@/pages/user/userForm';
import { HeaderWrapper, BodyWrapper, FooterWrapper, AntdButton, BlogLabelWrapper, FooterLink } from './styles';
import LoginForm from "@/pages/login/index";
import { ConnectState } from '@/models/connect';
import moment from 'moment';
import { message } from 'antd';

type MatchPathProps = {
    route: RouteProps,
    user: UserModelState,
    dispatch: Dispatch,
}
const MatchPath: FC<MatchPathProps> = ({ dispatch, route, user }) => {

    /** 跳转至新增页面 */
    const addBlog = useCallback(() => {
        const now = moment().subtract(30, 'm');
        const isValidToken = now.isBefore(user.tokenOutDate)
        // 验证token有效
        if (isValidToken) history.push({ pathname: '/articleEditedForm' });
        // token无效
        else {
            handleLogout()
            message.warn("身份认证过期，请重新登录！")
        }
    }, [])

    /** 跳转至登录页面 */
    const handleLogin = useCallback(() => {
        if (!user.isLogin) history.push({ pathname: '/login' });
        else handleLogout()
    }, [user.isLogin])

    /** 清除登录信息 */
    const handleLogout = useCallback(() => {
        dispatch({
            type: 'user/logout',
            payload: false
        })
    }, [])

    const buttonLabel = useMemo(() => user.isLogin ? "注销" : "登录", [user.isLogin])

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
            <HeaderWrapper>
                <BlogLabelWrapper>
                    {/* 梁凉凉 Blog */}
                </BlogLabelWrapper>
                <div>
                    {/* <Button onClick={goToAddUser}>新增用户</Button> */}
                    {user.isLogin && <AntdButton onClick={addBlog}>新增博客</AntdButton>}
                    <AntdButton onClick={handleLogin}>{buttonLabel}</AntdButton>
                </div>
            </HeaderWrapper>

            <BodyWrapper>
                {bodyComponent}
            </BodyWrapper>

            <FooterWrapper>
                <FooterLink onClick={() => window.open("https://beian.miit.gov.cn")}>
                    鲁ICP备2022022109号 © 2022 HY Liang
                </FooterLink >
            </FooterWrapper>
        </>
    );
}

export default connect(({ user }: ConnectState) => ({ user }))(MatchPath);