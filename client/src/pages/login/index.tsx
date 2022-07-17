import React, { FC, useCallback } from "react";
import { Form, Input, Button, message } from 'antd';
import { LoginParams } from './type';
import axios from "axios";
import { connect } from "dva";
import { ConnectState } from "@/models/connect";
import type { Dispatch, UserModelState } from 'umi';
import moment from "moment";

type LoginFormProps = {
    dispatch: Dispatch,
    user: UserModelState
}

type LoginResponse = {
    data: {
        token: string,
        expiresIn: number
    }
}

/** 登录 */
const LoginForm: FC<LoginFormProps> = ({ dispatch, user }) => {

    const [form] = Form.useForm();

    const login = useCallback(async () => {
        const formItemObj = await form.validateFields()
        if (!formItemObj) return
        const { username, password, } = formItemObj as LoginParams
        if (!username || !password) return
        axios
            .post("api/login", { username, password })
            .then((res: LoginResponse) => {
                if (res.data?.token && res.data?.expiresIn) {
                    localStorage.setItem('token', res.data.token);
                    // 计算过期时间
                    dispatch({
                        type: 'user/saveOutDate',
                        payload: moment().add(res.data.expiresIn, 's')
                    })
                    dispatch({
                        type: 'user/saveLoginState',
                        payload: true
                    })
                    message.success("登录成功");
                }
            })
            .catch((err) => { });
        // const data = await postRequest("login", { username, password })
        // if (!data) return
        // message.success("登录成功");
    }, [])

    return (
        <div>
            <Form form={form} name="basic">
                <Form.Item
                    label="用户名"
                    name="username"
                    rules={[{ required: true, message: '请输入用户名!' }]}
                >
                    <Input placeholder="请输入用户名" />
                </Form.Item>
                <Form.Item
                    label="密码"
                    name="password"
                    rules={[{ required: true, message: '请输入密码!' }]}
                >
                    <Input.Password placeholder="请输入密码" />
                </Form.Item>
            </Form>
            <Button onClick={login}>登录</Button>
        </div>
    )
}
export default connect(({ user }: ConnectState) => ({ user }))(LoginForm) 