
// AheadLibExDlg.h: 头文件
//

#pragma once
#include <vector>

typedef struct _EXPORT_FUNCTION
{
	BOOL isOrd;
	DWORD Ordinal;
	DWORD FunctionRVA;
	DWORD NameOrdinal;
	DWORD NameRVA;
	CString Name;

	IMAGE_SECTION_HEADER secInfo; //区段信息

	BOOL isUnkown;
	BOOL isFunc; //是否是函数
	BOOL isTranFunc; //是否是中转导出表
	BOOL isData; //是否是数据
	ULONG isDataCount; //导出数据大小，每一个指针当一个计数 
	CString TranName; //中转导出表名称

}EXPORT_FUNCTION, * PEXPORT_FUNCTION;

// CAheadLibExDlg 对话框
class CAheadLibExDlg : public CDialogEx
{
// 构造
public:
	CAheadLibExDlg(CWnd* pParent = nullptr);	// 标准构造函数

// 对话框数据
#ifdef AFX_DESIGN_TIME
	enum { IDD = IDD_AHEADLIBEX_DIALOG };
#endif

	protected:
	virtual void DoDataExchange(CDataExchange* pDX);	// DDX/DDV 支持


// 实现
protected:
	HICON m_hIcon;

	// 生成的消息映射函数
	virtual BOOL OnInitDialog();
	afx_msg void OnSysCommand(UINT nID, LPARAM lParam);
	afx_msg void OnPaint();
	afx_msg HCURSOR OnQueryDragIcon();
	DECLARE_MESSAGE_MAP()

public:
	CEdit m_editInputFile;
	CEdit m_editOutputFile;
	CEdit m_editInfo;
	CEdit m_editOutputProject;
	afx_msg void OnBnClickedButtonInputfile();
	afx_msg void OnBnClickedButtonOutputfile();
	afx_msg void OnDropFiles(HDROP hDropInfo);
	afx_msg void OnBnClickedRadioCpp();
	afx_msg void OnBnClickedButtonOutputProject();
	afx_msg void OnBnClickedRadioProject();
	afx_msg void OnBnClickedOk();

	void OnLoadFile();
	void OnAnalyzeFile();

	void OnCreateCppSource(CString & strSource, CString& strAsmSource);
	void OnCreateSln(CString& strSln);
	void OnCreateVcxproj(CString& strVcxproj);
	void OnCreateFilters(CString& strFilters);

public:
	BOOL m_bIsWow64;

	BOOL m_bIsx64;
	CString m_strFileName;			// xxx.dll
	CString m_strFilePath;			// C:\\windows\\xxx.dll
	CString m_strProjectPath;		// C:\\windows\\xxx/
	CString m_strFileNameNOExtension;			// xxx.cpp
	CString m_strAsmName;			// xxx_jump.asm

	CString m_strGuidProject;
	CString m_strGuidVcxproj;
	CString m_strGuidSolution;

	HMODULE m_hDll;
	std::vector<IMAGE_SECTION_HEADER> m_vecSectionHdrs;
	std::vector<EXPORT_FUNCTION> m_vecExportFunc;
};
