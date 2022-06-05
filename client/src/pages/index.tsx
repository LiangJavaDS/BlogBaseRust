
import React, { FC } from 'react';
import MatchPath from '@/components/MatchPath/index';
import { RouteProps } from 'umi/node_modules/@types/react-router';

type HomePageProps = {
  route: RouteProps
}
const HomePage: FC<HomePageProps> = (props) => {

  return (
    <>
      <MatchPath route={props.route} />
    </>
  );
}

export default HomePage