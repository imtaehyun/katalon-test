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

'상품 페이지 이동'
WebUI.navigateToUrl('http://qa-emart.ssg.com/item/itemView.ssg?itemId=0000008026713&siteNo=6001&salestrNo=2034')

'바로구매 버튼 클릭'
WebUI.click(findTestObject('Object Repository/order/a__1'))

isDuhagee = WebUI.verifyElementVisible(findTestObject('order2/jumun_duhagee'))

'최초주문일때'
if (isDuhagee == false) {
    WebUI.click(findTestObject('Object Repository/order/td_800g ()'))
} else {
    '주문더하기가 뜰때 새로 주문하기 버튼 클릭'
    WebUI.click(findTestObject('order2/new_order_button'))
}

try {
    '쓱배송일자 선택'
    WebUI.click(findTestObject('order/input_select_day_time_css'))

    '품절상품 대체여부 > 전체수락'
    WebUI.click(findTestObject('Object Repository/order/input_ _shortgProcMthdCdAll_1'))

    '계속하기 버튼 클릭'
    WebUI.click(findTestObject('Object Repository/order/button__1'))

    's포켓 전체사용 버튼 클릭'
    WebUI.click(findTestObject('Object Repository/order/button__2'))

    '"주문 상품정보 및 결제대행 서비스 이용약관에 모두 동의하십니까?" 클릭'
    WebUI.click(findTestObject('Object Repository/order/label_'))

    '결제하기 버튼 클릭'
    WebUI.click(findTestObject('Object Repository/order/button_0  0'))

    ord_no = WebUI.getText(findTestObject('order/ord_no_txt'))

    ord_no = ord_no.replace('주문번호는 ', '')

    ord_no = ord_no.replace('번', '')

    '주문번호 Global 변수에 저장\t'
    GlobalVariable.ord_no = ord_no

    System.out.println(ord_no)
}
catch (Exception e) {
    WebUI.takeScreenshot()
} 
finally { 
}

