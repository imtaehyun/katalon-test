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

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://qa-bo.ssgadm.com/authentication/login.ssg')

WebUI.setText(findTestObject('picking_dirc_admin/input__userId'), '140179')

WebUI.setEncryptedText(findTestObject('search_ord_from_bo/input__userPwd (1)'), 'w11JizTPrrG9r5NfjDZfNA==')

WebUI.click(findTestObject('search_ord_from_bo/a_LOGIN (1)'))

WebUI.click(findTestObject('search_ord_from_bo/a_ (1)'))

WebUI.click(findTestObject('search_ord_from_bo/a_E-Infra (1)'))

WebUI.click(findTestObject('search_ord_from_bo/b_E (1)'))

WebUI.setText(findTestObject('search_ord_from_bo/input__salestrNm'), '성수점')

WebUI.selectOptionByValue(findTestObject('search_ord_from_bo/select_(2034)'), '2034|성수점', false)

WebUI.selectOptionByValue(findTestObject('search_ord_from_bo/select_'), '20', true)

System.out.println(GlobalVariable.ord_no)

WebUI.setText(findTestObject('search_ord_from_bo/textarea'), GlobalVariable.ord_no)

WebUI.click(findTestObject('search_ord_from_bo/searchBtn'))

WebUI.delay(2)

WebUI.click(findTestObject('search_ord_from_bo/checkbox'))

WebUI.click(findTestObject('search_ord_from_bo/pickgDircProcBtn'))

WebUI.waitForAlert(5)

WebUI.acceptAlert()

WebUI.waitForAlert(5)

WebUI.acceptAlert()

