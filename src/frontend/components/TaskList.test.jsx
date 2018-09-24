import * as React from 'react'
import { shallow } from 'enzyme'
import TaskList from './TaskList'

it('renders the proper header', () => {
  let e = shallow(<TaskList tasks={[]} />)
  expect(e.find('.tasklist-header').text()).toEqual('(Un)Completed')
})
