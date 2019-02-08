import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.openBrowser('')

WebUI.setViewPortSize(1920, 1080)

WebUI.navigateToUrl('http://qa-www.ssg.com/')

txt_login_btn = WebUI.getText(findTestObject('order/a_'))

'비로그인 상태일때만 로그인 처리'
if (txt_login_btn == '로그인') {
    'GNB 로그인 버튼 클릭'
    WebUI.click(findTestObject('Object Repository/order/a_'))

    '팝업으로 focus 이동'
    WebUI.switchToWindowTitle('로그인, 신세계적 쇼핑포털 SSG.COM')

    WebUI.setText(findTestObject('Object Repository/order/input__mbrLoginId'), 'nezz')

    WebUI.setEncryptedText(findTestObject('Object Repository/order/input__password'), 'RKSwBYi2f48=')

    '로그인 버튼 클릭'
    WebUI.click(findTestObject('Object Repository/order/button_'))

    WebUI.switchToWindowTitle('신세계적 쇼핑포털 SSG.COM')
}

