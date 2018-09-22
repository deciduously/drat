import * as React from 'react'
import { shallow } from 'enzyme'
import Task from './Task'

it('renders the proper text', () => {
  let e = shallow(<Task title='TEST' />)
  expect(e.find('.title-text').text()).toEqual('TEST')
})
