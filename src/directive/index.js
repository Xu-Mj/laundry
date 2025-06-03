import copyText from './common/copyText'
import vSlideIn from './vSlideIn'

export default function directive(app) {
  app.directive('copyText', copyText)
  app.directive('slide-in', vSlideIn)
}