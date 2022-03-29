// Jyotsna: Design a struct that stores the response returned by Rekor after making a new entry (review the whole file)

/// Stores the response returned by Rekor after making a new entry
// get rid of default
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    // native uuid object maybe?
    uuid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    attestation: Option<Attestation>,
    body: String,
    integrated_time: i64,
    log_i_d: String,
    log_index: i64,
    verification: Verification,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attestation{
    // This field is just a place holder
    // Not sure what is stored inside the Attestation struct, it is empty for now
    #[serde(skip_serializing_if = "Option::is_none")]
    dummy: Option<String>,
}

/// Stores the signature over the artifact's logID, logIndex, body and integratedTime.
// get rid of default
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Verification {
    #[serde(skip_serializing_if = "Option::is_none")]
    inclusion_proof: Option<InclusionProof>,
    signed_entry_timestamp: String,
}

/// Stores the signature over the artifact's logID, logIndex, body and integratedTime.
// get rid of default
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InclusionProof {
    hashes:  Vec<String>,
    log_index: i64,
    root_hash: String,
    tree_size: i64,
}

/* 
Example response: 
{
	"uuid": "073970a07c978b7a9ff15b69fe15d87dfb58fd5756086e3d1fb671c2d0bd95c0",
	"attestation": {},
	"body": "eyJhcGlWZXJzaW9uIjoiMC4wLjEiLCJzcGVjIjp7ImRhdGEiOnsiaGFzaCI6eyJhbGdvcml0aG0iOiJzaGEyNTYiLCJ2YWx1ZSI6IjU0NDczODVjZjliOTIwNGE4YTRlNWVmMWI5NzFmNDAzMzQ5Yjk4OWM5ODcwZGE1NjAyNTk3NjhkNTNlYmQzZDAifX0sInNpZ25hdHVyZSI6eyJjb250ZW50IjoiTFMwdExTMUNSVWRKVGlCUVIxQWdVMGxIVGtGVVZWSkZMUzB0TFMwS0NtbFJTRXRDUVVGQ1EwRkJNRVpwUlVWalowTlZXRWMzT0dGa2FqWm9SMVZLU25KbVFtOUtNRFJ3U0c5R1FXd3ZPVEJOUlZkSVIzaHZZVmMxYTJNd1FuY0tZMjA1TUdJeU5YUlpWMnh6VEcxT2RtSlJRVXREVWtGdGREaEhaMjVVYVd0bGF6ZFRSRUZEV0VKNGVXcEROMHA0WTI5M2VWZE5kbmRtTVVkTWIwSXpad3BwVm5STldsQkRZazV6Umt0NGJuVktjemQwWlRSWGFsRnRiVUoxUWxOa1ZtazJabGRLU1hkSmRucEZjMVZpZWk5MVRsaFRVMlpEUkZwa1MwSXhNMEZDQ2tOdFZtTnBTRk5ITTBWd1ZuVjZUSGhXZEhObGIxb3djMlZwTkdjNGFHRkZWSEEzTVcxNUwwRkhTRVpuVkc1emExVnRjMWxwVG1wSlkyWm5LMVZTTHprS09IUnVkVmRXV0U1UVQyNHljbkJSZFZGaldqZE1lVXhqYjFCd1UyVnplbXAzTW1SaWVHdElVVXh5Y1U5eGJFcExNM2wyV1doelpubHNWa2xXTHpsemR3cHNjMlZxTm5SU00zUnZZWEJRWkdsMFJHbDRNV1EwYVhobE5EaGhTa1pxUzFKRlUzUk5SVFZOVWtWUk1XczVWRzFrVVZBMVQzUmtkbVF6UlRSTUwzVnZDbVp1ZG5GaFZ5OXNRVlJ5VWtkalR6bEJVVFZaYzJGS1JUZDZTSGxyYWtaTlFYUmFhMlZKTjA5U2JXeHpaRFpNZDIxR1dYRnRlblUxYzJadVJIbzRja2NLYjJzckwzWkhLMG93WjNCUVoyMDJhR1psUzFGYVlrazNSR1JwWkVONGIwVkRPRFZxU21JMlYzTjVUSEVyVWtkVVEzWTFlbVUwYjA1eFMwNW1kWFZyVHdwUmVqTnNVR0pJWml0RVRISk5aMHRFYkRKSWRYSkRTRVI2Yldad2RESXJTWFU1WkVsTWVGRnhOVWR5WTNWSFprRkxiRGM1Tms0eFpUUnRRemM0V0c1Q0NtaFRUelJ2VEhkVFRrVmFaaXRJWm1GWU5GTnNMMDV4Ukc5NFNrbGlhQ3MyTkRZek9HTjRNRDBLUFRsSlpYSUtMUzB0TFMxRlRrUWdVRWRRSUZOSlIwNUJWRlZTUlMwdExTMHRDZz09IiwiZm9ybWF0IjoicGdwIiwicHVibGljS2V5Ijp7ImNvbnRlbnQiOiJMUzB0TFMxQ1JVZEpUaUJRUjFBZ1VGVkNURWxESUV0RldTQkNURTlEU3kwdExTMHRDZ3A0YzBST1FrWXJZMGxOTUVKRVFVTmhPRWMzVWtReWRqTnRhWGROZEhoV1lWcHBNMHBDVm5WbFZrRnhTRXRETkdWTGIwMVROVWhOUTFKdkswaGFWbEpCQ2pjd1dHMXpWSEJZTVZveFoxcFJkWFZETUVkRVdUSTJhRUpvWldwQmNUTm9lREp5ZGpZdk9IRTVNRUoyVjBkSU9YUldaVWR3VERGellVbHROVEpuUlZJS1dIbFdaMmQ2Tld0QlF6QlROblpOYmpka2NqSmxkRUpyVjFkUUswOXFNRFZUTURKWldrSlVXV2Q0Y0U5aWVXVmpWVk5qY1V0T1ZHcHpiRnBSUWtneVpBcFRTSFZyTTI4eVdqZG9UVFE1VlRCc04zcGlWM2MwYjBsVUsyeEJVbU56WVdwUlZIZFhhbXhwWVZCRUwwaFNhbFF5YmxKUGFFbG9hWFJsWkM5M1ozbDZDbmxrU1hFMVpUWnpNVGhXVEdOVU56VnhWMjV5V2xoT1VGZEdkMll5TlZKWVdUTjFkR3RYSzBkWE5XNVJaVTQ0TUZFeWExSkZaMnQ0Um5NMVFXUTFWMW9LZGtVM2REZ3ZhSGcxZW0xemJGbzBkR1pHTkhOcE0xRmFaVWxSUW1GaldXSjNlRTFRVTBSbU9XOUdSM2hrUjBaVU9EZzVkMHBNUjJkWGJYSXhWR3RRVFFwalRqQTJkM2hCVWtkMGVFNHdlall4UmtwVWFXcE1WMUppYWxjemRXNUpPV2hqVVdOVmJFNHZVU3N4Tm05MFNIQmxTMVpuTkc5WE1EQkRjblpYVDBRMkNuRnJUR05OUkRRNWVWVkVPR1pUUjBJclVrVnVhVUpoT0RsRE9XdFJWVFJUUzJSbmMweE1MMUVyU2tzclUzazVTMjFKUkhKdE1XRTBSR1pRTVhCelptVUtUR3BoY25welZscG1TMVZJWm5kalFVVlJSVUZCWXpCcFZFaFdjbHBUUWtsaFZ6VnJZM2xCT0dKSGFIQmliVko2VVVoQ2VXSXpVblppYlRGb1lWZDNkUXBaTWpsMFVITk1Ra1pCVVZSQlVXZEJVR2haYUVKSVNVRnNSbmgxTDBkdVdTdHZVbXhEVTJFemQyRkRaRTlMVWpaQ1VVcG1ia05FVGtGb2MwUkNVV3RFQ25kdFkwRkNVWE5LUTBGalEwSm9WVXREVVdkTVFXZFJWMEZuVFVKQmFEUkNRV2hsUVVGQmIwcEZRMkV6ZDJGRFpFOUxValphTVd0TUx6RkpTekIyWkdVS1dsZzFjalZUWldKT2VGUkpUbE5CUVhaWmEzSkxVbmxLTldZM2JFOU5PV2RNUjBsMVl6SkdiMDVWYm1wV1VWUXdja2xIT1RBeE9XZzBPSEJEZVRreFpncFlha1JFVWsxWk9XZDZSbGRYUTJkSGJsaG9NV2hYU1ROTk4wSktSalpaUlRaMU5rUllSM04yZFZWd1IzSk9aVnBCUnpacmEyRjZRWFZCYm01V01HdERDakE0ZW05U2NrRmFRM1pzY0dGYWNubGtPR2wwWWl0eVZpdFJTM0EzUVhjeWJFRkpTREZsTm1SM1RUUlNURVpxZG1ack9FeEtXSGhxU2tGdlVHMTNObXdLVEhjeE9HTTNiMWMyVWt4UE9WRllVVGhsVFRaeU1uWklTSEJ0TUZSMVpIWmFlV0ZtVG5WRE16SkhSR3hOV1RSMU1GWXhSR0k0VEhONWJWQnpRV2gxUVFveVNubzBMMHRRY1RaMVMzZEpkRzFXU3pSd2JtUm1SVVIxTmtReFZHOXZSRmxZYVhCMFdXRm1aSFpWTXpOd1ZWRjRkMGh2WmxSVVprVTFlbHAzTWxCbENteElNMjVhWkhOblNGaEhVSGhLVEV4TmNVOXdWelJETDJOTk5scFJWbWRaVTNSV2NqQnVkbFUyTml0UmFsRjJjMnRWV2xJd05tUmtSWHB1UW5CSFNuTUtkSEJ0YWpsQlpTOUhVbGs0UlU1dVRqa3ZNa2RtUlhWeWRIb3paRXRPVlZwdmFrMTVNVFV6YW1OSE1GVXhlbnBvTVRFMVYwbzNkRGgzU0VKMU5GTTBjQW93WjBVclVrRnhlWFJCWTBsYVJHUXlUbE5PY25vNFZuSTVSa1U1ZUN0bVlYUTVSVkpzWW01a1FVSkZOV2xXT0hOTE1DdEdZVzVYZDJkak4wRjZVVkptQ201RFJFNUJVWGRCZEVKdmRHaG1ZMUo2Y2pONGNqTlFPWEEzVVVOTmQwdDFhVzl1ZGsxRGJUaFhaM2RPVXpSRGNHaHhielZPVDNJeWFVMXFhMHhRTUVvS2IyMW5Ta3hXV0RWT0sySnlkamg1TkVnNGNsbFFkMHRDTVRadkwyaEJPRWxpUjJKd1dYbHRNMFpqZVd0VWQyTmlWMkowVUZSTVJYUmtRMVZRVEZsVVJBcE9RelZNUjBwd1p6TmxPRFpaWmxGMFFVNDJMMDF1V25sWlQyMXNSSGd5VjBkMGRFeGtiWE5CVTBkV2RYZzJRVlpLY1VsMkszZ3dObFZMU2tWdFN6TjBDbXBzUlZaTGVXY3hNbEpGZW5sbE5VbFVObkZGVTBkd1QzcHZNbGxzVjFWeFNWUjNMMEZoVUZFeVduaFZZWGgyV1VadlZVOWpkMmRqWkc1SWEyZHphRWtLVDI0NWFDOU9TRlZ0VURNeVYxRjJjV3RSVFhWVllWQkpUbEp6UXpnelMzWlVSRWRzZVdaVFNGWkdlazFoTkdoRVRXaEZZMWg2TkdGamFXNWtOVmRVWlFwNmVVeG5XbWhQWWpkalRtVkRlRFI0WTNKMFVFSTJWVGRDVWk5R1ZreDZURUpzUVhwMWVtcHBSV2haZDBwdk0wRlBUWEZHYjFJMWJVRnhhR3gxZEU1UENuTnplVzltWW5GVVowZGlVMHhrYW1KWVVDOWhSWFJuZWpKTlZqbHVMMjlqTVZOQ09FaGxXazh2TVRkS2VXZHVlbkoxU1V0NUt5OXNUMWRQZW5RcmFsWUtWa1p3Vm5sb01YVmxPR3hHTjNsdFMxSTBkSE5zSzJsSlZtSnhibEIyY0Uxb1RFOUpRbkZZUm00eVowMURhMGR2U2t4NU4wOUliekpYUVVWS1IyeDBNd3BUZDNCaWNtcHFNVUZDUlVKQlFVaERkMUIzUlVkQlJVbEJRMWxYU1ZGU2VVRktVbU5pZG5od01sQnhSVnBSYTIxME9FZG5ibFJwYTJWblZVTllOWGRuQ25wUlNXSkVRVlZLUVRoS2JrRkJRVXREVWtGdGREaEhaMjVVYVd0bGFXNXBSRUZEUlVGbWExcHhMelJTY0RKaFRrRTBaR0p2U2pkVlJsaEVUMkZTYTFZS09VMUxiMFZhUm5GVVRVNXZka1JNTlhob1RXeG5iRkJRZFM5c0syUm9WR2Q0WkdWS09VVldTRzlsZW5SaU9EazJWUzl3VDNWQ1VuTnVPVlowVnpSWkx3cHFaV2xYTjBWNVRsaEJaQzlQY25adVJtSjRLemRwV0V4eGRYQmFTa3BHVkdrdmFqbFNhRlpaVG5OdGJEZHpaV0pVVUdWQ2JrZEVRVGt4Y1dKRE5IaElDbkJSVmtSRGRXcDROamxXZUU4MVJURk1VMjlvUTAwclR5ODFka3hDYlRocE1XOHZibUpHYldKNU4xWkRlVXRsVWtSbWFIUm1PVzVET0RSeGMwVTVSM0VLVlRjdlRGTnBhemxpWm5oTlYySndjVGg1YTI1MGJWTXpZVEJ6ZW1NMFlsWkdjR1Y2UW5CdFRtSXdRVlpqUWl0VWJUbG5WMjFGZW1ocFRITTJSa3RCVGdwSmJuRk9kVmgxUWt3NVVFTmhZemNyYlZVcll6SnRRbWRIVDFKSFpERmtXazh6VWtNNE9YcEdNM2hDUWxsdVEwOWxOV05CVFVac1l6RllSM05zYkhOSkNtUjZaSEprV0haaVRrSjZMMm8zTVhCMVRqaHZSbGx0TDFoaVZtTnBaVTh3VkdaUmFVUmpWSFE0UzJscFVqbFVRVVE1TDFBMU9UTlNUV3hNVDBkVE9IQUthSFpLWW1sR2IxcG1XRWhqYkhOYVJraHRPRVJSVVdFNU5FbGFkMVJDT0cwMFowSldNRTB5V0ZOMlpFaHZNekJzYzNGcWRGcGhXbWxUY2xKb05ISnphQXB1TVRSd1lrRmhWR1JoUzBWUVkzWjBkV1ppVlhWWE1FbHFXV1F5YTNCSlZDOTBaejBLUFdoYVdGVUtMUzB0TFMxRlRrUWdVRWRRSUZCVlFreEpReUJMUlZrZ1FreFBRMHN0TFMwdExRPT0ifX19LCJraW5kIjoicmVrb3JkIn0=",
	"integratedTime": 1610469905,
	"logID": "c0d23d6ad406973f9559f3ba2d1ca01f84147d8ffc5b8445c224f98b9591801d",
	"logIndex": 1,
	"verification": {
		"inclusionProof": {
			"hashes": ["b08416d417acdb0610d4a030d8f697f9d0a718024681a00fa0b9ba67072a38b5", "766ae2c918bbc083a6cce41f6ff3a3cf1a8153b86f594303ce16ded44c99647b", "d17f9cb9776767672a026878d31368d6620c12c0b6eb537c7788f975771d5488", "3d3952e9ac03dd00d6817cb1ba7e4ca832fdc9472fd066f6eb6550f4a6d3cff1", "2b5d2e57470fbaa592f6311b5665ec2870297c39dda407fe8ee45f146df27dbc", "8a0cd237f42edf76d50ac9a539108a4f0c73cefd87a27e5ede32ee5153d79a6c", "24fe0dc0e19a75cd7bd5f1a58c5aff086bbefcc6c777b70b21a87a5bfd8c8c23", "d0b0d0cf8fc7a914cad93607ac5bfd511afd8d9b3729da87cb4d243b53385f17", "7e777698d7e8ceb92a03c05cde3632ab12aa4fc3513064a723d68370b287c9d3", "0d0f2102d842d56843727e1848872655501af276613f8d04b6ccad806a334d57", "34f60d3658ce812525e34e8b6858678be09d9d44b8f9d8c61814be5706c615cf", "15ca1b46d6458b9eecd3e1669a8c2848449d158655de9d24f78fd70ad9955e3b", "6cde371d35dfc14a44ac42c1ed55025066a261d39333f1c7ff25b7955da48f2f", "6d96b436ff041cd3b73f9c291b223b2cc47ec13ab92f97a5d625d15f9aa439fb", "70ebf715c1f35f512c68cedbd83d47aba5e5ed309ddbba9472f1786e3b2e40c3", "7e6076f58c8241594c9592de95fa4d698b23985025a135049299abc901d5de03", "1891d104464a0d35c122f6b4a5ac1b146dc9f43dc86e86f0bc07491b1f50c410", "93b48fbede85f163c9a6c96ea5a4af34c212ae946b8870fdbcf9e61987fa5c3c", "df963b882a1c8375d0723e4f7e496752a62561f02da4755d2dff311e42ad2d16", "34300f7e815c2e7346b6a918b49bb8af51fa32aeada5e86e36d4c19fe0552e72", "385e0a22a9d292fe0c865754dafa7c35b3c13010d4c3b205e39a73900e1dba98"],
			"logIndex": 1,
			"rootHash": "91c2baa8cfcdb0303d3d6e3b23ba41fad6986711276f8cc91958d3d06306a63c",
			"treeSize": 1594684
		},
		"signedEntryTimestamp": "MEYCIQCQQ4h4mLmz1QxE8X6bsd7msh/2xxlVQsq/eQJlg9lZ2QIhAIO024d6TcCG5u55VUlPLCfqlRYGWCG6sXpjkRU88YFZ"
	}
} 
*/