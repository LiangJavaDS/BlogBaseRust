import { Reducer } from 'umi';

export type UserModelState = {
    isLogin?: boolean,
    tokenOutDate?: string
}
export type SaveAction = {
    payload: UserModelState
}
export type UserModelType = {
    namespace: 'user';
    state: UserModelState,
    reducers: {
        saveOutDate: Reducer<UserModelState>,
        saveLoginState: Reducer<UserModelState>,
        logout: Reducer<UserModelState>,
    }
}

const UserModel: UserModelType = {
    namespace: 'user',
    state: {
        tokenOutDate: undefined,
        isLogin: false
    },
    reducers: {
        saveOutDate(state, action) {
            return {
                ...state,
                tokenOutDate: action.payload ?? undefined
            }
        },
        saveLoginState(state, action) {
            return {
                ...state,
                isLogin: action.payload
            }
        },
        logout(state, action) {
            localStorage.removeItem("token")
            return {
                ...state,
                isLogin: action.payload
            }
        },
    },
};

export default UserModel