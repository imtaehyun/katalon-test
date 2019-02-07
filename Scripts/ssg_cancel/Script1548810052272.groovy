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

WebUI.navigateToUrl('https://qa-pay.ssg.com/myssg/orderInfo.ssg?viewType=Ssg')

WebUI.click(findTestObject('Object Repository/cancel/button_ (1)'))

WebUI.click(findTestObject('Object Repository/cancel/label_'))

WebUI.waitForElementVisible(findTestObject('cancel/label_ (1)'), 5)

WebUI.click(findTestObject('Object Repository/cancel/label_ (1)'))

WebUI.click(findTestObject('Object Repository/cancel/button_ (2)'))

WebUI.click(findTestObject('Object Repository/cancel/label_     .'))

WebUI.click(findTestObject('Object Repository/cancel/button__1'))

WebUI.click(findTestObject('Object Repository/cancel/button__2'))

